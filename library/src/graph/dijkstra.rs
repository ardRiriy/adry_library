use std::{cmp::Reverse, collections::BinaryHeap};

use crate::utils::integer::Integer;

#[allow(dead_code)]
pub struct Dijkstra<T> {
    start: usize,
    dist: Vec<T>,
    from: Vec<Option<usize>>,
}

impl<T: Integer> Dijkstra<T> {
    pub fn new(start: usize, graph: &Vec<Vec<(usize, T)>>) -> Self {
        let inf = T::inf();
        let v = graph.len();
        let mut distance = vec![inf; v];
        let mut from = vec![None; v];
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((T::zero(), start, start)));

        let mut left = v; // 辺の本数が多い場合の定数倍改善
        while let Some(Reverse((cost, pos, f))) = pq.pop() {
            if distance[pos] != inf {
                continue;
            }
            from[pos] = Some(f);  
            distance[pos] = cost;
            left -= 1;
            if left == 0 {
                break;
            }

            for &(ni, w) in &graph[pos] {
                if distance[ni] == inf {
                    pq.push(Reverse((w + cost, ni, pos)));
                }
            }
        }
        
        from[start] = None; // 実装上戻しておいたほうが都合がいい

        Self { start, from, dist: distance }
    }
    
    pub fn get(&self, to: usize) -> T {
        self.dist[to]
    }
    
    pub fn path(&self, to: usize) -> Option<Vec<usize>> {
        self.from[to]?;
        
        let mut res = vec![to];
        let mut cur = to;
        
        while let Some(p) = self.from[cur] {
            res.push(p);
            cur = p;
        }
        res.reverse();
        
        Some(res)
    }
}
