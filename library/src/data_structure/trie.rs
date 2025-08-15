static NEXT_NODE_SIZE: usize = 26;


#[derive(Clone, Debug)]
struct TrieNode {
    accepted_cnt: usize,
    next: Vec<Option<Box<TrieNode>>>,
}

impl TrieNode {
    fn new(accepted_cnt: usize) -> Self {
        // 初期化時, root nodeだけは1にしておきたいので設定可能にしておく
        Self { accepted_cnt, next: vec![None; NEXT_NODE_SIZE] }
    }
    
    fn add(&mut self, i: usize, s: &[char]) {
        if i < s.len() {
            let idx = (s[i] as u8 - b'a') as usize;
            if let Some(next_node) = self.next[idx].as_mut() {
                next_node.add(i+1, s);
            } else {
                let mut new_node = Self::new(0);
                new_node.add(i+1, s);
                self.next[idx] = Some(Box::new(new_node));
            }
        } else {
            self.accepted_cnt += 1;
        }
    }
    
    fn search(&self, i: usize, s: &[char]) -> Result<usize, ()> {
        if i == s.len() {
            return if self.accepted_cnt > 0 {
                Ok(self.accepted_cnt)
            } else {
                Err(())
            }
        }
        
        let idx = (s[i] as u8 - b'a') as usize;
        if let Some(next_node) = self.next[idx].as_ref() {
            next_node.search(i+1, s)
        } else {
            Err(())
        }
    }
}


#[derive(Debug)]
pub struct Trie {
    root: TrieNode,    
}

impl Trie {
    pub fn new() -> Self {
        Self { root: TrieNode::new(1) }
    }
    
    /* 
     * 単語Sを追加する 
     * O(|S|) 
    */
    pub fn insert(&mut self, s: &String) {
        self.root.add(0, &s.chars().collect::<Vec<char>>());
    }
    
    /*
     * 単語Sが存在するかどうかを判定
     * 存在する場合: Ok(単語数)
     * 存在しない場合: Err(())
     * O(|S|)
    */
    pub fn search(&self, s: &String) -> Result<usize, ()> {
        self.root.search(0, &s.chars().collect::<Vec<char>>())
    }
}