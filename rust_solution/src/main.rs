use std::collections::HashMap;
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

struct AutocompleteSystem {
    root: TrieNode,
}

impl AutocompleteSystem {
    fn new() -> Self {
        AutocompleteSystem {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, username: &str) {
        if username.is_empty() {
            return; // Do not mark the root as end of a word for an empty string
        }
        let mut node = &mut self.root;
        for c in username.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::new);
        }
        node.is_end_of_word = true;
    }

    fn contains(&self, username: &str) -> bool {
        let mut node = &self.root;
        for c in username.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return false,
            }
        }
        node.is_end_of_word
    }

    fn starts_with(&self, prefix: &str) -> Vec<String> {
        let mut results = Vec::new();
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return results,
            }
        }
        self.dfs(node, prefix, &mut results);
        results
    }

    fn dfs(&self, node: &TrieNode, prefix: &str, results: &mut Vec<String>) {
        if node.is_end_of_word {
            results.push(prefix.to_string());
        }
        for (c, child_node) in &node.children {
            let new_prefix = format!("{}{}", prefix, c);
            self.dfs(child_node, &new_prefix, results);
        }
    }
}

fn main() {
    let mut autocomplete_system = AutocompleteSystem::new();
    let usernames = ["alice", "albert", "bob", "carol", "caroline"];

    // Insert each username into the autocomplete system
    for username in usernames {
        autocomplete_system.insert(username);
    }

    println!("Contains 'caroline': {}", autocomplete_system.contains("caroline"));
    println!("Contains 'nope': {}", autocomplete_system.contains("nope"));

    let results = autocomplete_system.starts_with("car");
    println!("Usernames starting with 'car': {:?}", results);

    let results = autocomplete_system.starts_with("b");
    println!("Usernames starting with 'b': {:?}", results);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_with_usernames() -> AutocompleteSystem {
        let mut system = AutocompleteSystem::new();
        let usernames = ["alice", "albert", "bob", "carol", "caroline"];
        for &username in &usernames {
            system.insert(username);
        }
        system
    }

    #[test]
    fn test_starts_with_specific() {
        let system = setup_with_usernames();

        // Test with prefix "al"
        let mut al_results = system.starts_with("al");
        al_results.sort();
        let mut expected_al = vec!["alice", "albert"];
        expected_al.sort();
        assert_eq!(al_results, expected_al);

        // Test with prefix "car"
        let mut car_results = system.starts_with("car");
        car_results.sort();
        let mut expected_car = vec!["carol", "caroline"];
        expected_car.sort();
        assert_eq!(car_results, expected_car);

        // Test with a prefix that should have no matches
        let no_results = system.starts_with("xyz");
        assert!(no_results.is_empty());
    }

    #[test]
    fn test_insert_empty_string() {
        let mut system = setup_with_usernames();
        system.insert("");
        assert!(!system.contains(""), "Inserting empty string should not mark root as end of word");
    }

    #[test]
    fn test_search_empty_string() {
        let system = setup_with_usernames();
        assert!(!system.contains(""), "Searching for empty string should return false");
    }

    #[test]
    fn test_autocomplete_empty_string() {
        let system = setup_with_usernames();
        let results = system.starts_with("");

        // Assuming starts_with("") should return all usernames
        let mut expected_results = vec!["alice", "albert", "bob", "carol", "caroline"];
        expected_results.sort();

        let mut results_sorted = results.clone();
        results_sorted.sort();

        assert_eq!(results_sorted, expected_results, "Autocomplete for empty string should return all usernames");
    }
}