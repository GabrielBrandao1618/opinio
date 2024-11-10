use std::{iter::Peekable, str::Chars};

use crate::words::{github::GithubWords, WordsProvider};

mod words;

struct CompletionNode {
    pub token: char,
    branches: Vec<CompletionNode>,
}

impl CompletionNode {
    pub fn new(token: char) -> Self {
        Self {
            token,
            branches: vec![],
        }
    }
    pub fn add_branch_from_word(&mut self, word: &str) {
        // TODO: Indicate whether a branch is a word ending of not, so we don't just compute the
        // longer completion result
        let first_char = word.chars().next();
        if let Some(first) = first_char {
            if let Some(existing_branch) = self.branch_by_char(first) {
                existing_branch.add_branch_from_word(&word[1..word.len()]);
            } else {
                let mut new_branch = CompletionNode::new(first);
                new_branch.add_branch_from_word(&word[1..word.len()]);
                self.branches.push(new_branch);
            }
        }
    }
    pub fn branch_by_char(&mut self, branch: char) -> Option<&mut CompletionNode> {
        for root in &mut self.branches {
            if root.token == branch {
                return Some(root);
            }
        }
        None
    }
    fn branch_by_word_chars(
        &mut self,
        mut word: Peekable<Chars<'_>>,
    ) -> Option<&mut CompletionNode> {
        if let Some(first_char) = word.next() {
            let found_branch = self.branch_by_char(first_char);
            if let Some(_) = word.peek() {
                if let Some(branch) = found_branch {
                    return branch.branch_by_word_chars(word);
                }
            } else {
                return found_branch;
            }
        }
        None
    }
    pub fn branch_by_word(&mut self, word: &str) -> Option<&mut CompletionNode> {
        let word_chars = word.chars().peekable();
        let found_branch = self.branch_by_word_chars(word_chars);
        return found_branch;
    }
    pub fn load_dataset(&mut self, dataset: Vec<String>) {
        for word in &dataset {
            self.add_branch_from_word(word);
        }
    }
    pub fn get_all_suggestions(&self) -> Vec<String> {
        if self.branches.len() == 0 {
            return vec![self.token.to_string()];
        }
        let mut completions: Vec<String> = vec![];
        for branch in &self.branches {
            for completion in branch.get_all_suggestions() {
                completions.push(format!("{a}{completion}", a = self.token));
            }
        }

        completions
    }
    pub fn get_word_suggestions(&mut self, word: &str) -> Vec<String> {
        if let Some(target_branch) = self.branch_by_word(word) {
            return target_branch.get_all_suggestions();
        }
        vec![]
    }
}

#[tokio::main]
async fn main() {
    let mut tree = CompletionNode::new('0');
    let words = GithubWords::get_words().await;
    tree.load_dataset(words);
    let suggestions = tree.get_word_suggestions("appl");
    for suggestion in suggestions {
        println!("{suggestion}");
    }
}
