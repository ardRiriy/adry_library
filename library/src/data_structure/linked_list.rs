#[derive(Clone, Copy, Debug)]
pub struct Item<T> {
    previous: Option<usize>,
    next: Option<usize>,
    val: Option<T>,
}

impl<T> Item<T> 
where T: Copy
{
    pub fn new(x: Option<T>) -> Self {
        Item { previous: None, next: None, val: x }
    }

    // dummy nodeが呼ばれないことを期待
    pub fn get(&self) -> T {
        assert!(self.val.is_some());
        self.val.unwrap()
    }

    pub fn update_prev(&mut self, prev: Option<usize>) {
        self.previous = prev;
    }

    pub fn update_next(&mut self, next: Option<usize>) {
        self.next = next;
    }

    pub fn next(&self) -> Option<usize> {
        self.next
    }

    pub fn previous(&self) -> Option<usize> {
        self.previous
    }
}

pub struct LinkedList<T> {
    vec: Vec<Item<T>>,
    head: usize,
    tail: usize,
}

impl<T> Default for LinkedList<T>
where T: Copy
 {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> 
where T: Copy
{
    pub fn new() -> Self {
        let dummy = Item::new(None);
        LinkedList { vec: vec![dummy], head: 0, tail: 0 }
    }

    // idxの後ろに要素を差し込み後、生成されたNodeのindexを返却
    pub fn add_back(&mut self, idx: usize, x: T) -> usize {
        let len = self.vec.len();
        let mut node = Item::new(Some(x));
        node.update_prev(Some(idx));

        if let Some(i) = self.vec[idx].next() {
            node.update_next(Some(i));
            self.vec[i].update_prev(Some(len));
        } else {
            self.tail = len;
        }
        self.vec[idx].update_next(Some(len));
        len
    }

    // idxの前に要素を差し込み後、生成されたNodeのindexを返却
    pub fn add_front(&mut self, idx: usize, x: T) -> usize {
        let len = self.vec.len();
        let mut node = Item::new(Some(x));
        node.update_next(Some(idx));
        if let Some(i) = self.vec[idx].previous() {
            node.update_prev(Some(i));
            self.vec[i].update_next(Some(len));
        } else {
            self.head = len;
        }
        self.vec[idx].update_prev(Some(len));
        len
    }

    // idx番のnodeを削除する(といいつつ、実際には消さないのでMLEには注意)
    pub fn delete(&mut self, idx: usize) {
        let deleted_node = self.vec[idx];
        
        // 前の処理
        if let Some(i) = deleted_node.previous() {
            self.vec[i].update_next(deleted_node.next());
        } else {
            assert!(deleted_node.next().is_some());
            self.head = deleted_node.next().unwrap();
        }

        // 後ろの処理
        if let Some(i) = deleted_node.next() {
            self.vec[i].update_prev(deleted_node.previous());
        } else {
            assert!(deleted_node.previous().is_some());
            self.tail = deleted_node.previous().unwrap();
        }
    }
}
