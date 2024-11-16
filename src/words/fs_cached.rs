use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use crate::words::WordsProvider;

pub struct FsCachedWords<P: WordsProvider> {
    provider: P,
}
impl<'a, P: WordsProvider> FsCachedWords<P> {
    pub fn new(words_provider: P) -> Self {
        Self {
            provider: words_provider,
        }
    }
    fn read_cache_file<R: AsRef<Path>>(file_path: R) -> Option<Vec<String>> {
        if let Ok(temp_cached_file) = OpenOptions::new().read(true).open(file_path) {
            let mut words = vec![];
            let mut word_buf = Vec::<u8>::new();
            let mut r = BufReader::new(temp_cached_file);
            while let Ok(readed_bytes) = r.read_until(b'\n', &mut word_buf) {
                if readed_bytes == 0 {
                    break;
                }
                if word_buf[word_buf.len() - 1] == b'\n' {
                    words.push(
                        String::from_utf8_lossy(&word_buf[0..word_buf.len() - 1]).to_string(),
                    );
                    word_buf.clear();
                }
            }
            drop(r);

            if words.len() > 0 {
                Some(words)
            } else {
                None
            }
        } else {
            None
        }
    }
    fn save_cache_file<R: AsRef<Path>>(file_path: R, words: &Vec<String>) {
        if let Ok(temp_cached_file) = OpenOptions::new().write(true).open(file_path) {
            let mut w = BufWriter::new(temp_cached_file);
            for word in words {
                let _ = w.write_all(&format!("{}\n", word).as_bytes());
            }
            let _ = w.flush();
        }
    }
    fn get_cache_file_path() -> PathBuf {
        let temp_dir_path = std::env::temp_dir();
        let temp_app_dir_path = temp_dir_path.join("opinio");
        if !temp_app_dir_path.is_dir() {
            let _ = std::fs::create_dir(&temp_app_dir_path);
        }
        let temp_file_path = temp_app_dir_path.join("cache");
        if !temp_file_path.is_file() {
            let _ = std::fs::File::create(&temp_file_path).unwrap();
        }
        temp_file_path
    }
}

impl<P: WordsProvider> WordsProvider for FsCachedWords<P> {
    async fn get_words(&self) -> Vec<String> {
        let temp_file_path = Self::get_cache_file_path();
        if let Some(cached_words) = Self::read_cache_file(&temp_file_path) {
            return cached_words;
        }
        let words = self.provider.get_words().await;
        let _ = Self::save_cache_file(&temp_file_path, &words);
        words
    }
}
