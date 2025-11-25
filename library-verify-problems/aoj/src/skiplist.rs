use library::{
    data_structure::skip_list::SkipList,
    utils::{input::Input, iterlibs::strs::StrUtilIter},
};

fn main() {
    let mut ip = Input::new();
    let q = ip.next::<usize>();

    let mut list = SkipList::new();
    let mut cur = 1;

    for _ in 0..q {
        let t: i8 = ip.next();
        match t {
            0 => {
                let x = ip.next::<i64>();
                list.insert(cur - 1, x);
            }
            1 => {
                let d = ip.next::<isize>();
                if d > 0 {
                    cur = cur + d as usize;
                } else {
                    cur = cur - d.abs() as usize;
                }
            }
            2 => {
                list.delete(cur - 1);
            }
            _ => {}
        }
    }
    println!(
        "{}",
        (0..list.len()).map(|i| list.get(i).unwrap()).join("\n")
    );
}
