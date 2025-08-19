use std::{str::{from_utf8, FromStr}};

pub struct Input {
    buf: Vec<u8>,
    pos: usize,
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl Input {
    pub fn new() -> Self {
        Self {
            buf: Vec::new(),
            pos: 0,
        }
    }

    pub fn next<T: FromStr>(&mut self) -> T {
        while self.pos < self.buf.len() && self.buf[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        let start = self.pos;
        while self.pos < self.buf.len() && !self.buf[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        
        if start == self.pos {
            // ioを追加で読む
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            self.buf.clear();
            self.buf.extend(input.as_bytes());
            self.pos = 0;
            return self.next();
        }

        from_utf8(&self.buf[start..self.pos])
            .unwrap()
            .parse::<T>()
            .ok()
            .unwrap_or_else(|| panic!("Failed to parse input: {}",
                from_utf8(&self.buf[start..self.pos]).unwrap()))
    }

    #[allow(non_snake_case)]
    pub fn vector<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }

    pub fn graph(&mut self, n: usize, m: usize, is_one_way: bool) -> Vec<Vec<usize>> {
        let mut graph = vec![Vec::new(); n];
        for _ in 0..m {
            let (u, v): (usize, usize) = self.pair();
            graph[u - 1].push(v - 1);
            if !is_one_way {
                graph[v - 1].push(u - 1);
            }
        }
        graph
    }

    pub fn weighted_graph<T: Copy + FromStr>(
        &mut self,
        n: usize,
        m: usize,
        is_one_way: bool,
        is_one_based: bool,
    ) -> Vec<Vec<(usize, T)>> {
        let mut graph = vec![Vec::new(); n];
        for _ in 0..m {
            let (u, v, w): (usize, usize, T) = self.triple();
            let u = if is_one_based { u - 1 } else { u };
            let v = if is_one_based { v - 1 } else { v };
            graph[u].push((v, w));
            if !is_one_way {
                graph[v].push((u, w));
            }
        }
        graph
    }

    pub fn pair<T: FromStr, U: FromStr>(&mut self) -> (T, U) {
        (self.next(), self.next())
    }

    pub fn triple<T: FromStr, U: FromStr, V: FromStr>(&mut self) -> (T, U, V) {
        (self.next(), self.next(), self.next())
    }
}