use lazy_static::lazy_static;
use std::fmt::Debug;

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
pub enum WordClass {
    N,
    V,
    Adj,
    Adv,
    Par,
    Pron,
    Num,
    Art,
    Prep,
    Conj,
    Interj,
    #[default]
    Unknown,
}

lazy_static! {
    pub static ref WORD_CLASS_ITEM: Vec<WordClass> = {
        vec![
            WordClass::N,
            WordClass::V,
            WordClass::Adj,
            WordClass::Adv,
            WordClass::Par,
            WordClass::Pron,
            WordClass::Num,
            WordClass::Art,
            WordClass::Prep,
            WordClass::Conj,
            WordClass::Conj,
            WordClass::Interj,
            WordClass::Unknown,
        ]
    };
    pub static ref WORD_CLASS_STR: Vec<String> = {
        vec![
            "n.", "v.", "adj.", "adv.", "par.", "pron.", "num.", "art.", "prep.", "conj.", "interj.",
            "unknown.",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
    };
}

impl WordClass {
    fn from_string(s: String) -> WordClass {
        if let Some((index, _)) = WORD_CLASS_STR
            .iter()
            .enumerate()
            .find(|(_, &ref x)| x == &s)
        {
            WORD_CLASS_ITEM[index]
        } else {
            WordClass::Unknown
        }
    }

    pub fn from_string_to_list<S: AsRef<str>>(s: S) -> Vec<WordClass> {
        let mut re = s.as_ref()
            .split(';')
            .map(|i| Self::from_string(i.to_string()))
            .collect::<Vec<WordClass>>();
        re.pop();

        re
    }

    pub fn wordclass_to_string(wc: &WordClass) -> String {
        if let Some((index, _)) = WORD_CLASS_ITEM
            .iter()
            .enumerate()
            .find(|(_, &ref x)| x == wc)
        {
            WORD_CLASS_STR[index].clone()
        } else {
            WORD_CLASS_STR.last().unwrap().clone()
        }
    }

    pub fn wordclass_list_to_string(wcs: &Vec<WordClass>) -> String {
        wcs.iter()
            .map(|wc| {
                let mut s = WordClass::wordclass_to_string(wc);
                s.push(';');
                s
            })
            .collect::<Vec<String>>()
            .join("")
    }
}

#[derive(Default, Debug)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub word_class: Vec<WordClass>,
    pub meaning: String,
    pub review_count: i32,
    pub correct_count: i32,
    pub last_review_date: String,
    pub display_order: i32,
}

#[cfg(test)]
mod tests {
    use crate::word_model::WordClass;

    #[test]
    fn from_string_test0() {
        let wc = WordClass::from_string_to_list(&"n.;v.;adj.;".to_string());
        assert_eq!(wc, vec![WordClass::N, WordClass::V, WordClass::Adj]);
    }

    #[test]
    fn from_string_test1() {
        let wc = WordClass::from_string_to_list(&"n.;".to_string());
        assert_eq!(wc, vec![WordClass::N]);
    }

    #[test]
    fn wordclass_to_string() {
        assert_eq!(
            WordClass::wordclass_to_string(&WordClass::Adv),
            "adv.".to_string()
        );
    }

    #[test]
    fn wordclass_list_to_string() {
        // println!(
        //     "{:?}",
        //     WordClass::wordclass_list_to_string(&vec![WordClass::N, WordClass::Adv, WordClass::V])
        // );
        assert_eq!(
            WordClass::wordclass_list_to_string(&vec![WordClass::N, WordClass::Adv, WordClass::V]),
            "n.;adv.;v.;".to_string()
        );
    }
}
