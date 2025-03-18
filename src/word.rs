use scraper::{Html, Selector};
use crate::definition::{to_part, Definition, Part};

/// Represents a dictionary entry for a word.
#[derive(Debug)]
pub struct Word {
    word: String,
    pinyin: String,
    definitions: Vec<Definition>,
}

impl Word {
    /// Create a new word entry by parsing the HTML content.
    pub fn parse(entry: &str, word: String) -> Self {
        let document = Html::parse_document(entry);
        let pinyin_selector = Selector::parse(".pinyin-text").unwrap();
        let pinyin = document
            .select(&pinyin_selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("");
        let definitions = parse_definitions(&document);
        Word {
            word,
            pinyin: pinyin.to_string(),
            definitions,
        }
    }

    /// Print the word entry.
    pub fn print(&self) {
        println!("Word: {}", self.word);
        println!("Pinyin: {}", self.pinyin);
        for definition in &self.definitions {
            println!("{}", definition.to_string());
        }
    }
}

fn parse_definitions(document: &Html) -> Vec<Definition> {
    // obtain the list of definitions
    let definition_selector = Selector::parse(".definition-box").unwrap();
    let definition_boxes = document.select(&definition_selector);
    let mut definitions: Vec<Definition> = Vec::new();

    // iterate through definitions to extract data
    for definition in definition_boxes {
        // extract the meaning text from the definition box
        let meaning_selector = Selector::parse(".word").unwrap();
        let meaning = definition
            .select(&meaning_selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>()
            .join("");

        // extract the part of speech image url from the definition box
        let part_selector = Selector::parse(".cixing-img-item").unwrap();
        let mut part: Option<Part> = None;
        if let Some(part_element) = definition.select(&part_selector).next() {
            let part_url = part_element.value().attr("src").unwrap();
            part = to_part(part_url);
        }

        // create a new definition object and add it to the list
        let definition = Definition::new(part, meaning);
        definitions.push(definition);
    }

    definitions
}
