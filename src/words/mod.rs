pub mod fs_cached;
pub mod github;

pub trait WordsProvider {
    async fn get_words(&self) -> Vec<String>;
}
