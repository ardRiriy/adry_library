use library::{algorithm::max_flow::MaxFlow, utils::input::Input};

fn solve(ip: &mut Input) {
    let (v, e) = ip.pair::<usize>();
    let mut flow = MaxFlow::new(v, None);

    for _ in 0..e {
        let (u, v) = ip.pair::<usize>();
        let c = ip.next::<u64>();
        flow.add_edge(u, v, c);
    }
    let ans = flow.execute();
    println!("{}", ans);
}

fn main() {
    let mut ip = Input::new();
    solve(&mut ip);
}
