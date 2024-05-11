use std::path::Path;

use chrono::Local;
use rusqlite::Connection;

use crate::word_builder::WordBuilder;
use crate::word_model::{Word, WordClass};

fn value_passer<T>(t: T) -> T {
    t
}

macro_rules! define_update_function {
    ($name:ident, $typ:ty) => {
        define_update_function!($name, $typ, value_passer);
    };

    ($name:ident, $typ:ty, $con:expr) => {
        paste::item! {
            #[allow(non_snake_case)]
            fn [< update_ $name _by_ $typ >](&mut self, id: i32, value: $typ) -> Result<(), rusqlite::Error> {
                self.context[id as usize].$name = $con(value.clone());
                let mut stmt = self.conn.prepare(concat!("update Words set ", stringify!($name), " = ? where id = ?;"))?;
                stmt.execute(&[&value.to_string(), &id.to_string()])?;
                Ok(())
            }
        }
    }
}

fn get_words_from_conn(conn: &Connection) -> Result<Vec<Word>, rusqlite::Error> {
    let mut words = Vec::<Word>::new();
    let mut stmt = conn.prepare(
        r"
               select
               id,
               word,
               word_class,
               meaning,
               review_count,
               correct_count,
               last_review_date,
               display_order
               from Words
               ",
    )?;

    let word_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<usize, i32>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, String>(2)?,
            row.get::<usize, String>(3)?,
            row.get::<usize, i32>(4)?,
            row.get::<usize, i32>(5)?,
            row.get::<usize, String>(6)?,
            row.get::<usize, i32>(7)?,
        ))
    })?;

    for word in word_iter {
        let word = word?;
        let w = WordBuilder::new()
            .id(word.0)
            .word(word.1)
            .word_classes(&WordClass::from_string_to_list(&word.2))
            .meaning(word.3)
            .review_count(word.4)
            .correct_count(word.5)
            .last_review_date(word.6)
            .display_order(word.7)
            .build();
        words.push(w);
    }

    Ok(words)
}

pub struct DBConnection {
    conn: Connection,
    context: Vec<Word>,
}

impl DBConnection {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<DBConnection, rusqlite::Error> {
        let conn = get_connection(path)?;
        let context = get_words_from_conn(&conn)?;

        Ok(DBConnection { conn, context })
    }

    pub fn insert_words(&mut self, words: Vec<Word>) -> Result<(), rusqlite::Error> {
        self.conn.execute("begin TRANSACTION;", [])?;
        let mut stmt = self.conn.prepare(
            r"insert into Words
            (word, word_class, meaning, review_count, correct_count, last_review_date, display_order)
            values (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
        )?;

        for word in words.iter() {
            stmt.execute(&[
                &word.word,
                &WordClass::wordclass_list_to_string(&word.word_class),
                &word.meaning,
                &word.review_count.to_string(),
                &word.correct_count.to_string(),
                &word.last_review_date,
                &word.display_order.to_string(),
            ])?;
        }

        self.conn.execute("commit;", [])?;
        self.context.extend(words);

        Ok(())
    }

    pub fn delete_word(&mut self, id: i32) -> Result<(), rusqlite::Error> {
        self.conn
            .prepare("delete from Words where id = ?;")?
            .execute(&[&id.to_string()])?;
        let _ = self.context.remove(id as usize);

        Ok(())
    }

    pub fn update_to_current_date(&mut self, id: i32) -> Result<(), rusqlite::Error> {
        let format_date = Local::now().format("%Y-%m-%d").to_string();
        let mut stmt = self
            .conn
            .prepare(r"update Words set last_review_date = ? where id = ?;")
            .unwrap();
        stmt.execute(&[&format_date, &id.to_string()])?;
        self.context[id as usize].last_review_date = format_date;

        Ok(())
    }

    pub fn update_review_count(
        &mut self,
        id: i32,
        is_correct: bool,
    ) -> Result<(), rusqlite::Error> {
        self.update_to_current_date(id)?;
        let id = id as usize;
        self.context[id].review_count += 1;
        if is_correct {
            self.context[id].correct_count += 1;
        }

        let mut stmt = self.conn.prepare(
            "update Words set review_count = ?, correct_count = ?, last_review_date = ? where id = ?;",
        )?;
        stmt.execute(&[
            &self.context[id].review_count.to_string(),
            &self.context[id].correct_count.to_string(),
            &self.context[id].last_review_date.to_string(),
            &id.to_string(),
        ])?;

        Ok(())
    }

    define_update_function!(word, String);
    define_update_function!(word_class, String, WordClass::from_string_to_list);
    define_update_function!(meaning, String);
    define_update_function!(last_review_date, String);
    define_update_function!(display_order, i32);
}

fn get_connection<P: AsRef<Path>>(path: P) -> Result<Connection, rusqlite::Error> {
    Connection::open(path)
}

#[cfg(test)]
mod tests {
    use chrono::Local;
    use std::env;

    use crate::db_util::DBConnection;
    use crate::file_util::get_or_create_resource_dir;
    use crate::word_builder::WordBuilder;
    use crate::word_model::WordClass;

    fn get_test1_db() -> DBConnection {
        let _ = get_or_create_resource_dir().unwrap();
        DBConnection::from_path(
            &*env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("qiancizhan-resources/test1.db"),
        )
        .unwrap()
    }

    #[test]
    fn insert_words_test() {
        let mut db = get_test1_db();

        db.insert_words(vec![WordBuilder::new()
            .word("juice".to_string())
            .word_class(WordClass::N)
            .meaning("果汁".to_string())
            .display_order(4)
            .build()])
            .unwrap();

        assert_eq!(db.context.last().unwrap().word, "juice");
    }

    #[test]
    fn macro_test() {
        let mut db = get_test1_db();
        db.update_word_by_String(1, "banananananana".to_string())
            .unwrap();
        assert_eq!(db.context[1].word, "banananananana");
    }

    #[test]
    fn update_to_current_date_test() {
        let mut db = get_test1_db();
        db.update_to_current_date(2).unwrap();
        let format_date = Local::now().format("%Y-%m-%d").to_string();
        assert_eq!(db.context[2].last_review_date, format_date);
    }

    #[test]
    fn update_review_count_test() {
        let mut db = get_test1_db();
        db.update_review_count(1, true).unwrap();
        assert_eq!(db.context[1].review_count, 1);
        assert_eq!(db.context[1].correct_count, 1);
    }

    #[test]
    fn delete_word_test() {
        let mut db = get_test1_db();
        db.delete_word(4).unwrap();
        assert_ne!(db.context[4].word, "watermelon");
    }
}
