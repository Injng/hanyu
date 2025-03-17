/// The url containing the picture of the noun part of speech used in the definition box.
const NOUN_URL: &str = "https://edu-public.cdn.bcebos.com/cms_data/2024-7/1721650227756/81d2e002ab72.png";

/// The url containing the picture of the verb part of speech used in the definition box.
const VERB_URL: &str = "https://edu-public.cdn.bcebos.com/cms_data/2024-7/1721650168465/764a32d2e3d8.png";

/// The url containing the picture of the adjective part of speech used in the definition box.
const ADJECTIVE_URL: &str = "https://edu-public.cdn.bcebos.com/cms_data/2024-7/1721650132932/17d61d430ef2.png";

/// Represents a part of speech for the word's definition.
#[derive(Debug)]
pub enum Part {
    Noun,
    Verb,
    Adjective
}

/// Represents a definition for a word.
#[derive(Debug)]
pub struct Definition {
    pub part: Option<Part>,
    pub meaning: String,
}

impl Definition {
    pub fn new(part: Option<Part>, meaning: String) -> Self {
        Definition {
            part,
            meaning,
        }
    }
}

pub fn to_part(url: &str) -> Option<Part> {
    match url {
        NOUN_URL => Some(Part::Noun),
        VERB_URL => Some(Part::Verb),
        ADJECTIVE_URL => Some(Part::Adjective),
        // unreachable
        _ => None,
    }
}