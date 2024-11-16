use crate::words::WordsProvider;

pub struct GithubWords {}
impl GithubWords {
    pub fn new() -> Self {
        Self {}
    }
}

impl WordsProvider for GithubWords {
    async fn get_words(&self) -> Vec<String> {
        let mut words = vec![];
        let response = reqwest::get(
            "https://raw.githubusercontent.com/dwyl/english-words/refs/heads/master/words.txt",
        )
        .await;
        if let Ok(res) = response {
            if let Ok(text) = res.text().await {
                let split_words = text.split("\n");
                split_words.for_each(|word| words.push(word.to_owned().to_ascii_lowercase()));
            }
        }
        words
    }
}
