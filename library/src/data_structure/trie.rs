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
    
    fn delete_one(&mut self, i: usize, s: &[char]) -> usize {
        if i == s.len() {
            if self.accepted_cnt == 0 {
                return 0;
            } else {
                self.accepted_cnt -= 1;
                return 1;
            }
        }
        
        let idx = (s[i] as u8 - b'a') as usize;
        return if let Some(next_node) = self.next[idx].as_mut() {
            next_node.delete_one(i+1, s)
        } else {
            0
        }
    }
    
    fn delete_all(&mut self, i: usize, s: &[char]) -> usize {
        if i == s.len() {
            let res = self.accepted_cnt;
            self.accepted_cnt = 0;
            return res;
        }
        
        let idx = (s[i] as u8 - b'a') as usize;
        return if let Some(next_node) = self.next[idx].as_mut() {
            next_node.delete_all(i+1, s)
        } else {
            0
        };
    }
    
    fn clean_up(&mut self, i: usize, s: &[char]) -> bool {
        if i == s.len() {
            return self.accepted_cnt == 0 && self.next.iter().all(|node| node.is_none());
        }
        
        let idx = (s[i] as u8 - b'a') as usize;
        if let Some(next_node) = self.next[idx].as_mut() {
            let ret = next_node.clean_up(i+1, s);
            if ret {
                self.next[idx] = None;
                return self.accepted_cnt == 0 && self.next.iter().all(|node| node.is_none());
            }
        }
        false
    }
    
    fn lcp(&self, i: usize, s: &[char]) -> usize {
        if i == s.len() {
            return i;
        }
        let idx = (s[i] as u8 - b'a') as usize;
        if let Some(next_node) = self.next[idx].as_ref() {
            next_node.lcp(i+1, s)
        } else {
            i
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
    
    
    /*
     * 単語Sを削除
     * is_delete_allなら全て、そうでないなら1つ
     * 削除できた場合: Ok(削除した個数)
     * 削除できなかった場合(該当の単語がない場合): Err(())
     * (|S|)
     * */
    pub fn delete(&mut self, s: &String, is_delete_all: bool) -> Result<usize, ()> {
        let s = &s.chars().collect::<Vec<char>>();
        let res = if is_delete_all {
            self.root.delete_all(0, s)
        } else {
            self.root.delete_one(0, s)
        };
        
        if res == 0 { 
            Err(()) 
        } else { 
            self.root.clean_up(0, s);
            Ok(res) 
        }
    }
    
    /*
     * Trieに登録されている文字列の集合とSのLCPを返す
     * O(|S|)
     * */
    pub fn lcp(&self, s: &String) -> usize {
        self.root.lcp(0, &s.chars().collect::<Vec<char>>())
    }
}