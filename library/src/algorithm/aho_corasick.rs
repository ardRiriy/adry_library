use std::collections::{HashMap, VecDeque};

use crate::utils::iterlibs::dedup::DedupIterator;

#[derive(Debug, Clone)]
struct Node {
    children: HashMap<char, usize>,
    failure: usize,
    outputs: Vec<usize>,
}

impl Node {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            failure: 0,
            outputs: Vec::new(),
        }
    }
}

pub struct AhoCorasick {
    nodes: Vec<Node>,
    patterns: Vec<String>,
}

impl AhoCorasick {
    pub fn new(patterns: &Vec<String>) -> Self {
        let mut res = Self {
            nodes: vec![Node::new()],
            patterns: patterns.clone(),
        };

        res.build_trie();
        res.build_failure();

        res
    }

    fn build_trie(&mut self) {
        for (pattern_idx, pattern) in self.patterns.iter().enumerate() {
            let mut cur = 0;

            for ch in pattern.chars() {
                let nxt = if let Some(nxt) = self.nodes[cur].children.get(&ch) {
                    *nxt
                } else {
                    let new_idx = self.nodes.len();
                    self.nodes.push(Node::new());

                    self.nodes[cur].children.insert(ch, new_idx);
                    new_idx
                };
                cur = nxt;
            }
            self.nodes[cur].outputs.push(pattern_idx);
        }
    }

    fn build_failure(&mut self) {
        let mut que = VecDeque::new();

        que.push_back(0);

        while let Some(cur) = que.pop_front() {
            for (&ch, &child) in &self.nodes[cur].children.clone() {
                que.push_back(child);
                let mut failure_node = self.nodes[cur].failure;
                while failure_node != 0 && !self.nodes[failure_node].children.contains_key(&ch) {
                    failure_node = self.nodes[failure_node].failure;
                }

                if let Some(&nxt) = self.nodes[failure_node].children.get(&ch) {
                    if nxt != child {
                        self.nodes[child].failure = nxt;
                        let ext = self.nodes[nxt].outputs.clone();
                        self.nodes[child].outputs.extend(ext);
                    }
                }
            }
        }
    }

    /* 与えられた文字列に登録されている文字列のうちi番目のものが含まれ、それがj文字目から始まるとき、(i, j)の組を返す */
    pub fn search(&self, s: &String) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        let mut cur = 0;

        for (p, ch) in s.char_indices() {
            while cur != 0 && !self.nodes[cur].children.contains_key(&ch) {
                cur = self.nodes[cur].failure
            }

            if let Some(&nxt) = self.nodes[cur].children.get(&ch) {
                cur = nxt;
            }

            for &pattern_idx in &self.nodes[cur].outputs {
                let start = p + 1 - self.patterns[pattern_idx].len();
                res.push((pattern_idx, start));
            }
        }
        res
    }    
    
    pub fn node_size(&self) -> usize {
        self.nodes.len()
    }
    
    /* sのsuffixに続く(root node以外へ)遷移するノード番号の一覧を返す */
    pub fn destination_node(&self, s: &String) -> Vec<usize> {
        let mut res = Vec::new();
        let mut cur = 0;
        
        for ch in s.chars() {
            while cur != 0 && !self.nodes[cur].children.contains_key(&ch) {
                cur = self.nodes[cur].failure;
            }

            if let Some(&nxt) = self.nodes[cur].children.get(&ch) {
                cur = nxt;
            } else {
                continue; // 遷移できない場合はスキップ
            }
        }
        
        while cur != 0 {
            for i in self.nodes[cur].children.values() {
                res.push(*i);
            }
            cur = self.nodes[cur].failure;
        }

        return res.iter().dedup().copied().collect();
    }

}
