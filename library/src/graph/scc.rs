use std::collections::VecDeque;

use crate::utils::chlibs::ChLibs;

pub struct Scc {
    graph: Vec<Vec<usize>>,
}

impl Scc {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        Self { graph }
    }

    pub fn execute(&self) -> Vec<Vec<usize>> {
        let n = self.graph.len();
        let mut scc_ids = vec![None; n];
        let mut current_id = 0;
        let mut current_order = 0;
        let mut order = vec![None; n];
        let mut stack = VecDeque::new();

        for v in 0..n {
            if order[v].is_none() {
                self.scc(
                    v,
                    &mut current_id,
                    &mut scc_ids,
                    &mut current_order,
                    &mut order,
                    &mut stack,
                );
            }
        }

        let mut groups = vec![Vec::new(); current_id];
        for (node, id) in scc_ids.into_iter().enumerate() {
            groups[id.expect("all nodes must have an scc id after run")].push(node);
        }
        groups
    }

    // その頂点から到達可能な最小のorder idを返す
    // SCC rootとなる頂点からの戻り値はNone
    fn scc(
        &self,
        u: usize,
        current_scc_id: &mut usize,
        scc_ids: &mut Vec<Option<usize>>,
        current_order: &mut usize,
        order: &mut Vec<Option<usize>>,
        stack: &mut VecDeque<usize>,
    ) -> Option<usize> {
        order[u] = Some(*current_order);
        *current_order += 1;
        stack.push_back(u);
        let mut res = order[u];

        for &v in self.graph[u].iter() {
            if scc_ids[v].is_some() {
                continue;
            }
            let candidate = order[v]
                .or_else(|| self.scc(v, current_scc_id, scc_ids, current_order, order, stack));
            if let Some(odr) = candidate {
                res = Some(res.map_or(odr, |r| r.min(odr)));
            }
        }

        if res.is_none_or(|x| x == order[u].unwrap()) {
            while let Some(v) = stack.pop_back() {
                scc_ids[v] = Some(*current_scc_id);
                if v == u {
                    break;
                }
            }
            *current_scc_id += 1;
            None
        } else {
            res
        }
    }
}
