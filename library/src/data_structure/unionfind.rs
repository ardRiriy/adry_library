#[derive(Debug)]
pub struct UnionFind<T, F>
where F: Fn(&T, &T) -> T,
{
    vertex: Vec<usize>,
    data: Vec<Option<T>>,
    merge_op: F,
}

impl<T: Clone, F: Fn(&T, &T) -> T> UnionFind<T, F> {
    pub fn new(size: usize, merge_op: F) -> Self {
        UnionFind {
            vertex: vec![!1; size] ,
            data: vec![None; size],
            merge_op,
        }
    }

    pub fn leader(&mut self, u: usize) -> usize {
        let elm: usize = self.vertex[u];
        if elm > self.vertex.len() {
            // 親ノードなので返却
            u
        } else {
            // 子ノードなので親ノードを取得したのち、圧縮
            self.vertex[u] = self.leader(elm);
            self.vertex[u]
        }
    }

    pub fn same(&mut self, u: usize, v: usize) -> bool {
        self.leader(u) == self.leader(v)
    }

    pub fn size(&mut self, u: usize) -> usize {
        let idx: usize = self.leader(u);
        !self.vertex[idx]
    }

    pub fn merge(&mut self, u: usize, v: usize) -> bool {
        if self.same(u, v) {
            // すでに親が同じ場合は何もせず、「マージされなかった」ことを報告
            return false;
        }
        let u_leader = self.leader(u);
        let u_size = self.size(u);

        let v_leader = self.leader(v);
        let v_size = self.size(v);

        let merged_size = !(u_size + v_size);
        if u_size < v_size {
            self.vertex[v_leader] = merged_size;
            self.vertex[u_leader] = v_leader;

            self.data[v_leader] = match (&self.data[u_leader], &self.data[v_leader]) {
                (Some(du), Some(dv)) => Some((self.merge_op)(du, dv)),
                (None, None) => None,
                _ => { unreachable!(); }
            };
        } else {
            self.vertex[u_leader] = merged_size;
            self.vertex[v_leader] = u_leader;

            self.data[u_leader] = match (&self.data[u_leader], &self.data[v_leader]) {
                (Some(du), Some(dv)) => Some((self.merge_op)(du, dv)),
                (None, None) => None,
                _ => { unreachable!(); }
            };
        }

        true
    }

    pub fn insert_data(&mut self, u: usize, value: T) {
        let root = self.leader(u);
        self.data[root] = Some(value);
    }

    pub fn get_data(&mut self, u: usize) -> Option<&T> {
        let root = self.leader(u);
        self.data[root].as_ref()
    }

}
