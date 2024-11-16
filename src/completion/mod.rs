use std::{iter::Peekable, str::Chars};

pub struct CompletionNode {
    pub token: char,
    branches: Vec<CompletionNode>,
    is_ending: bool,
}

impl CompletionNode {
    pub fn new(token: char) -> Self {
        Self {
            token,
            branches: vec![],
            is_ending: false,
        }
    }
    pub fn add_branch_from_word_chars(&mut self, mut word: Peekable<Chars<'_>>) {
        if let Some(first_char) = word.next() {
            if let Some(existing_branch) = self.branch_by_char(first_char) {
                existing_branch.add_branch_from_word_chars(word);
            } else {
                let mut new_branch = CompletionNode::new(first_char);
                new_branch.add_branch_from_word_chars(word);
                self.branches.push(new_branch);
            }
        } else {
            self.is_ending = true
        }
    }
    fn add_branch_from_word(&mut self, word: &str) {
        let word_chars = word.chars().peekable();
        self.add_branch_from_word_chars(word_chars);
    }
    fn branch_by_char(&mut self, branch: char) -> Option<&mut CompletionNode> {
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
    fn branch_by_word(&mut self, word: &str) -> Option<&mut CompletionNode> {
        let word_chars = word.chars().peekable();
        let found_branch = self.branch_by_word_chars(word_chars);
        return found_branch;
    }
    pub fn load_dataset(&mut self, dataset: Vec<String>) {
        for word in &dataset {
            self.add_branch_from_word(word);
        }
    }
    fn get_all_suggestions_recur(&self) -> Vec<String> {
        if self.branches.len() == 0 {
            return vec![self.token.to_string()];
        }
        let mut completions: Vec<String> = vec![];
        for branch in &self.branches {
            for completion in branch.get_all_suggestions_recur() {
                completions.push(format!("{a}{completion}", a = self.token));
            }
        }
        if self.is_ending {
            completions.push(self.token.to_string());
        }

        completions
    }
    fn get_all_suggestions(&self) -> Vec<String> {
        let suggestions = self
            .get_all_suggestions_recur()
            .into_iter()
            .map(|sugg| sugg[1..sugg.len()].to_string())
            .collect();
        suggestions
    }
    pub fn get_word_suggestions(&mut self, word: &str) -> Vec<String> {
        if let Some(target_branch) = self.branch_by_word(word) {
            return target_branch.get_all_suggestions();
        }
        vec![]
    }
}
