use std::{fmt::Debug, iter::repeat};

use crate::{misc::rand::Pcg32, utils::iterlibs::collect::CollectIter};

#[derive(Debug, Clone, Copy)]
struct ListNode {
    next: Option<usize>,
    element_node_idx: usize,
    width: usize, // 幅
}

impl ListNode {
    fn new(idx: usize) -> Self {
        Self {
            next: None,
            element_node_idx: idx,
            width: 0,
        }
    }

    fn dummy(idx: usize) -> Self {
        Self {
            next: None,
            element_node_idx: idx,
            width: 0,
        }
    }
}

const HEIGHT: usize = 25;

#[derive(Debug)]
struct ElementNode<T> {
    list_nodes: Vec<usize>,
    elm: Option<T>,
}

impl<T> ElementNode<T> {
    fn new(x: T) -> Self {
        Self {
            list_nodes: vec![],
            elm: Some(x),
        }
    }

    fn dummy() -> Self {
        Self {
            list_nodes: vec![],
            elm: None,
        }
    }
}

#[derive(Debug)]
pub struct SkipList<T> {
    nodes: Vec<ElementNode<T>>,
    list: Vec<Vec<ListNode>>,
    len: usize,
}

impl<T: Copy + Debug> SkipList<T> {
    /// Creates a new empty skip list.
    ///
    /// # Examples
    /// ```
    /// let list: SkipList<i32> = SkipList::new();
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn new() -> Self {
        let mut nodes = Vec::with_capacity(1 << 10);
        let mut head = ElementNode::dummy();
        head.list_nodes = repeat(0).take(HEIGHT).collect();
        nodes.push(head);

        let mut list = (0..HEIGHT)
            .rev()
            .map(|i| Vec::with_capacity(1 << i))
            .collect_vec();

        for i in 0..HEIGHT {
            let mut head_node = ListNode::dummy(0);
            head_node.next = Some(1);
            head_node.width = 1;
            list[i].push(head_node);
        }

        let mut tail = ElementNode::dummy();
        tail.list_nodes = repeat(1).take(HEIGHT).collect();
        nodes.push(tail);

        for i in 0..HEIGHT {
            let tail_node = ListNode::dummy(1);
            list[i].push(tail_node);
        }

        Self {
            nodes,
            list,
            len: 0,
        }
    }

    /// リストのp番目の要素を返却する。存在しない場合はNoneを返す。計算量: 平均 O(log N)
    pub fn get(&mut self, p: usize) -> Option<T> {
        if p >= self.len() {
            return None;
        }
        let mut cur_height = HEIGHT - 1;
        let mut cur_list_idx = 0;
        let mut cur_idx = !0;
        loop {
            let node = &self.list[cur_height][cur_list_idx];
            if node.width.wrapping_add(cur_idx) > p {
                cur_height -= 1;
                cur_list_idx = self.nodes[node.element_node_idx].list_nodes[cur_height];
            } else {
                cur_idx = node.width.wrapping_add(cur_idx);
                cur_list_idx = node.next.unwrap();
            }

            if cur_idx == p {
                break;
            }
        }

        self.nodes[self.list[cur_height][cur_list_idx].element_node_idx].elm
    }

    /// リストのp番目に要素を挿入する。計算量: 平均 O(log N)
    pub fn insert(&mut self, p: usize, elm: T) {
        // 対象となるp-1番目のelement nodeを見つける
        let target_list = self.get_target_list(p);

        let mut new_elm_node = ElementNode::new(elm);
        let mut is_already_ended = false;
        let mut pcg = Pcg32::new();
        for i in 0..HEIGHT {
            let is_create_new_node = i == 0 || (!is_already_ended && pcg.gen_range(0..=1) == 0);

            let prev_elm = &self.nodes[self.list[i][target_list[i].0].element_node_idx];

            if is_create_new_node {
                let new_list_node_id = self.list[i].len();
                new_elm_node.list_nodes.push(new_list_node_id);

                let mut new_node = ListNode::new(self.nodes.len());

                let prev_node = &mut self.list[i][prev_elm.list_nodes[i]];

                new_node.width = prev_node.width.wrapping_add(target_list[i].1) + 1 - p;
                new_node.next = prev_node.next;

                prev_node.next = Some(new_list_node_id);
                prev_node.width = p.wrapping_sub(target_list[i].1);

                self.list[i].push(new_node);
            } else {
                is_already_ended = true;
                self.list[i][prev_elm.list_nodes[i]].width += 1;
            }
        }

        self.nodes.push(new_elm_node);
        self.len += 1;
    }

    /// p番目の要素を削除し、削除したノードの値を返す。計算量: 平均O(log N)
    pub fn delete(&mut self, p: usize) -> Option<T> {
        if p >= self.len() {
            return None;
        }
        let target_list = self.get_target_list(p);
        let mut res = None;

        for i in 0..HEIGHT {
            if self.list[i][target_list[i].0]
                .width
                .wrapping_add(target_list[i].1)
                == p
            {
                if res.is_none() {
                    let idx =
                        self.list[i][self.list[i][target_list[i].0].next.unwrap()].element_node_idx;
                    res = self.nodes[idx].elm;
                }
                // 削除操作
                let nxt = self.list[i][self.list[i][target_list[i].0].next.unwrap()];
                self.list[i][target_list[i].0].next = nxt.next;
                self.list[i][target_list[i].0].width += nxt.width - 1;
            } else {
                self.list[i][target_list[i].0].width -= 1;
            }
        }
        self.len -= 1;
        res
    }

    // helper関数
    // res[i]には、高さiであって、挿入/削除対象のノードの直前に来るノードの
    // (self.list[i]上のindex, 実際のindex)
    // を格納して返す
    fn get_target_list(&self, p: usize) -> Vec<(usize, usize)> {
        if p > 0 {
            // i番目の高さに挿入処理が起こった場合、更新が必要なノードのリスト
            let mut previous_list_idxs = vec![(p - 1, p - 1); HEIGHT];

            let mut cur_height = HEIGHT - 1;
            let mut cur_list_idx = 0;
            let mut cur_idx = !0; // いくつ進めたか
            let targ = p - 1;

            loop {
                let node = &self.list[cur_height][cur_list_idx];
                if node.width.wrapping_add(cur_idx) > targ {
                    // heightを一段降りる
                    previous_list_idxs[cur_height] = (cur_list_idx, cur_idx);

                    cur_height -= 1;
                    cur_list_idx = self.nodes[node.element_node_idx].list_nodes[cur_height];
                } else {
                    // 次のノードへ進む
                    cur_idx = node.width.wrapping_add(cur_idx);
                    cur_list_idx = node.next.unwrap();
                }
                if cur_idx == targ {
                    loop {
                        previous_list_idxs[cur_height] = (cur_list_idx, cur_idx);
                        if cur_height == 0 {
                            break;
                        }
                        cur_height -= 1;
                        cur_list_idx = self.nodes
                            [self.list[cur_height + 1][cur_list_idx].element_node_idx]
                            .list_nodes[cur_height];
                    }
                    break;
                }
            }
            previous_list_idxs
        } else {
            vec![(0, !0); HEIGHT]
        }
    }

    /// リストに登録されている要素数を返す。計算量: O(1)
    pub fn len(&self) -> usize {
        self.len
    }

    //pub fn push(&mut self) -> Self {}
}
