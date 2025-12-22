use std::collections::VecDeque;

use crate::utils::{chlibs::ChLibs, consts::INF, integer::Integer};

pub struct MaxFlow<T> {
    g: Vec<Vec<T>>,
    start: usize,
    end: usize,
}

impl<T: Integer> MaxFlow<T> {
    pub fn new(n: usize, start_end: Option<(usize, usize)>) -> Self {
        let (start, end) = start_end.unwrap_or((0, n-1));

        Self { g: vec![vec![T::from_i32(0); n]; n], start, end }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, c: T) {
        self.g[u][v] += c;
    }

    pub fn execute(&mut self) -> T {
        let n = self.g.len();
        let sum = self.g[0].iter()
            .fold(T::from_i32(0), |acc, c| acc + *c);

        let mut dist = vec![T::from_u64(INF); n];

        loop {
            self.bfs(&mut dist);
            if dist[self.end] == T::from_u64(INF) {
                break;
            }

            loop {
                let path = self.dfs(&dist);
                if path.is_empty() {
                    break;
                }

                let mut flow = T::from_u64(INF);
                for i in 0..path.len()-1 {
                    flow.chmin(self.g[path[i]][path[i+1]]);
                }

                for i in 0..path.len()-1 {
                    self.g[path[i]][path[i+1]] -= flow;
                    self.g[path[i+1]][path[i]] += flow;
                }
            }
            dist.fill(T::from_u64(INF));
        }

        let d = self.g[0].iter()
            .fold(T::from_i32(0), |acc, c| acc + *c);

        sum-d
    }

    fn bfs(&self, dist: &mut Vec<T>) {
        dist[self.start] = T::from_i32(0);
        let mut que = VecDeque::new();
        que.push_back(self.start);
        while let Some(u) = que.pop_front() {
            for (v, ci) in self.g[u].iter().enumerate() {
                if ci <= &T::from_i32(0) || dist[v] != T::from_u64(INF) { 
                    continue; 
                }
                dist[v] = dist[u] + T::from_i32(1);
                que.push_back(v);
            }
        }
    }

    fn dfs(&self, dist: &Vec<T>) -> Vec<usize> {
        let n = self.g.len();
        let mut parent = vec![None; n];
        let mut stk = Vec::new();
        stk.push(self.start);

        while let Some(u) = stk.pop() {
            for (v, ci) in self.g[u].iter().enumerate() {
                if parent[v].is_some() || ci <= &T::from_i32(0) || dist[u+1] != dist[v] {
                    continue;
                } 
                parent[v] = Some(u);
                stk.push(v);
            }
        }

        let mut res = vec![];
        res.push(self.end);
        let mut cur = self.end;
        while let Some(u) = parent[cur] {
            res.push(u);
            cur = u;
        }

        res.reverse();
        res
    }

}

