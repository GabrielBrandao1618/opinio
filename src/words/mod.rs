pub mod github;

pub trait WordsProvider {
    async fn get_words() -> Vec<String>;
}
