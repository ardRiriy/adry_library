// verification-helper: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_B&lang=ja
use library::{
    graph::dfs::{dfs, DfsHandler},
    utils::{
        input::Input,
        iterlibs::{collect::CollectIter, strs::StrUtilIter},
    },
};

struct S {
    graph: Vec<Vec<usize>>,
    timestamp: u32,
    appears: Vec<Option<u32>>,
    done: Vec<u32>,
}

impl DfsHandler for S {
    type State = usize;

    fn neighbors(&mut self, state: &Self::State) -> Vec<Self::State> {
        self.graph[*state].clone()
    }

    fn is_visited(&self, state: &Self::State) -> bool {
        self.appears[*state].is_some()
    }

    fn mark_visited(&mut self, state: &Self::State) {
        self.appears[*state] = Some(self.timestamp);
        self.timestamp += 1;
    }
    fn on_leave(&mut self, state: &Self::State) {
        self.done[*state] = self.timestamp;
        self.timestamp += 1;
    }
}

fn solve(g: &Vec<Vec<usize>>) {
    let appears = vec![None; g.len()];
    let done = vec![!0; g.len()];
    let mut s = S {
        graph: g.clone(),
        timestamp: 1,
        appears,
        done,
    };
    dfs(&mut s, 0..g.len());
    println!(
        "{}",
        (0..g.len())
            .map(|i| format!("{} {} {}", i + 1, s.appears[i].unwrap(), s.done[i]))
            .join("\n")
    );
}

fn main() {
    let mut ip = Input::new();
    let n = ip.next();

    let g = (0..n).fold(vec![vec![]; n], |mut g, _| {
        let u = ip.next::<usize>() - 1;
        let k = ip.next();
        g[u] = ip
            .vector::<usize>(k)
            .into_iter()
            .map(|i| i - 1)
            .collect_vec();
        g
    });
    solve(&g);
}
