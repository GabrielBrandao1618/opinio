use words::fs_cached::FsCachedWords;

use crate::words::{github::GithubWords, WordsProvider};

mod completion;
mod words;

use completion::CompletionNode;

#[tokio::main]
async fn main() {
    let mut cli_args = std::env::args();
    if let Some(word_arg) = cli_args.nth(1) {
        let mut tree = CompletionNode::new('0');
        let words_provider = FsCachedWords::new(GithubWords::new());
        let words = words_provider.get_words().await;
        tree.load_dataset(words);
        let suggestions = tree.get_word_suggestions(&word_arg);
        for suggestion in suggestions {
            println!("{word_arg}{suggestion}");
        }
    } else {
        println!("No word was provided");
    }
}
