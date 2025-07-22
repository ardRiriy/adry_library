#[allow(dead_code)]
pub struct Dijkstra<T> {
    start: usize,
    dist: Vec<T>,
    from: Vec<Option<usize>>,
}

impl<T: Copy + num::PrimInt> Dijkstra<T> {
    pub fn new(start: usize, graph: &Vec<Vec<(usize, T)>>) -> Self {
        let inf = T::max_value() >> 2;
        let v = graph.len();
        let mut distance = vec![inf; v];
        let mut from = vec![None; v];
        let mut pq = std::collections::BinaryHeap::new();
        pq.push(std::cmp::Reverse((T::zero(), start, start)));

        let mut left = v; // 辺の本数が多い場合の定数倍改善
        while let Some(std::cmp::Reverse((cost, pos, f))) = pq.pop() {
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
                    pq.push(std::cmp::Reverse((w + cost, ni, pos)));
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
        if self.from[to].is_none() {
            return None;
        }
        
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
