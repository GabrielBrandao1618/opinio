use words::fs_cached::FsCachedWords;

use crate::words::{github::GithubWords, WordsProvider};

mod completion;
mod words;

use completion::CompletionNode;

#[tokio::main]
async fn main() {
    let mut tree = CompletionNode::new('0');
    let words_provider = FsCachedWords::new(GithubWords::new());
    let words = words_provider.get_words().await;
    tree.load_dataset(words);
    let suggestions = tree.get_word_suggestions("drag");
    for suggestion in suggestions {
        println!("{suggestion}");
    }
}
