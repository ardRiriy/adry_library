pub trait WFelm<T> 
where T: Copy
{
    fn min(&self, a: T, b: T) -> T; // コストが軽い方を返却
    fn add(&self, a: T, b: T) -> T; // コストの合算を計算
    fn infinity(&self) -> T;
    fn identity(&self) -> T;
}

pub struct DefaultWFelm;

impl<T> WFelm<T> for DefaultWFelm
where T: Copy + num::PrimInt {
    fn min(&self, a: T, b: T) -> T {
        a.min(b)
    }

    fn add(&self, a: T, b: T) -> T {
        if a == self.infinity() || b == self.infinity() {
            self.infinity()
        } else {
            a + b
        }
    }

    fn infinity(&self) -> T {
        T::max_value()
    }

    fn identity(&self) -> T {
        T::zero()
    }
}

pub struct WarshallFloyd<T, O>
where
    T: Copy,
    O: WFelm<T>,
{
    dist: Vec<Vec<T>>,
    op: O,
}

impl<T, O> WarshallFloyd<T, O>
where
    T: Copy,
    O: WFelm<T>,
{
    pub fn new(graph: &Vec<Vec<(usize, T)>>, op: O) -> Self {
        let n = graph.len();
        let mut dist = vec![vec![op.infinity(); n]; n];
        
        for i in 0..n {
            for (to, cost) in graph[i].iter() {
                dist[i][*to] = op.min(dist[i][*to], *cost);
            }
            dist[i][i] = op.identity();
        }
        
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    dist[i][j] = op.min(dist[i][j], op.add(dist[i][k], dist[k][j]));
                }
            }
        }
        
        Self { dist, op }
    }
    
    pub fn get(&self, from: usize, to: usize) -> T {
        self.dist[from][to]
    }
    
    pub fn add(&mut self, x: usize, y: usize, w: T) {
        self.dist[x][y] = self.op.min(self.dist[x][y], w);
        self.dist[y][x] = self.op.min(self.dist[y][x], w);
        for &k in &[x, y] {
            for i in 0..self.dist.len() {
                for j in 0..self.dist.len() {
                    self.dist[i][j] = self.op.min(self.dist[i][j], self.op.add(self.dist[i][k], self.dist[k][j]));
                }
            }
        }
    }
}

