use crate::word_model::{Word, WordClass};

pub struct WordBuilder {
    word: Word,
}

impl WordBuilder {
    pub fn new() -> WordBuilder {
        WordBuilder {
            word: Word::default(),
        }
    }

    pub fn id(mut self, id: i32) -> WordBuilder {
        self.word.id = id;
        self
    }

    pub fn word(mut self, word: String) -> WordBuilder {
        self.word.word = word;
        self
    }

    pub fn word_class(mut self, wc: WordClass) -> WordBuilder {
        self.word.word_class.push(wc);
        self
    }

    pub fn word_classes(mut self, wcs: &Vec<WordClass>) -> WordBuilder {
        self.word.word_class = wcs.iter().cloned().collect();
        self
    }

    pub fn meaning(mut self, meaning: String) -> WordBuilder {
        self.word.meaning = meaning;
        self
    }

    pub fn review_count(mut self, review_count: i32) -> WordBuilder {
        self.word.review_count = review_count;
        self
    }

    pub fn correct_count(mut self, correct_count: i32) -> WordBuilder {
        self.word.correct_count = correct_count;
        self
    }

    pub fn last_review_date(mut self, last_review_date: String) -> WordBuilder {
        self.word.last_review_date = last_review_date;
        self
    }

    pub fn display_order(mut self, display_order: i32) -> WordBuilder {
        self.word.display_order = display_order;
        self
    }

    pub fn build(self) -> Word {
        self.word
    }
}

#[cfg(test)]
mod tests {
    use crate::word_builder::WordBuilder;
    use crate::word_model::WordClass;

    #[test]
    fn build_test() {
        let word = WordBuilder::new()
            .id(1)
            .word("test".to_string())
            .word_class(WordClass::N)
            .meaning("测试".to_string())
            .review_count(0)
            .correct_count(0)
            .last_review_date("2020-01-01".to_string())
            .display_order(0)
            .build();

        assert_eq!(word.id, 1);
        assert_eq!(word.word, "test");
        assert_eq!(word.word_class, vec![WordClass::N]);
        assert_eq!(word.meaning, "测试");
        assert_eq!(word.review_count, 0);
        assert_eq!(word.correct_count, 0);
        assert_eq!(word.last_review_date, "2020-01-01");
        assert_eq!(word.display_order, 0);
    }
}
