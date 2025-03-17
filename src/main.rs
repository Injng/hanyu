mod word;
mod definition;

use headless_chrome::Browser;
use crate::word::Word;

fn main() {
    // get command line arguments for word to search
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <word>", args[0]);
        std::process::exit(1);
    }
    let word: String = args[1].clone();

    // initialize headless browser and tab
    let browser = Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();
    let dict_url = format!("https://hanyu.baidu.com/hanyu-page/term/detail?wd={}", word);

    // navigate to the page and obtain content
    tab.navigate_to(&dict_url).unwrap();
    tab.wait_for_element("div.pinyin-text").unwrap();
    let content = tab.get_content().unwrap();

    // parse content and print
    let word = Word::parse(&content, word);
    println!("{:?}", word);
}
