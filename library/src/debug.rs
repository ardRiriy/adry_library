pub trait Debuggable {
    fn debug_string(&self) -> String;
}

impl<T: std::fmt::Debug + std::fmt::Display> Debuggable for Vec<T> {
    fn debug_string(&self) -> String {
        use itertools::Itertools;
        use std::iter::repeat;
        if let Some(max_size) = self.iter()
            .enumerate()
            .map(|(i, x)| (format!("{:?}", x).len()).max(format!("{:?}", i).len()))
            .max() {
                let mut idx = String::from("idx |");   
                let mut val = String::from("val |");   
                for (i, xi) in self.iter().enumerate() {
                    idx.push_str(
                        &format!(" {:>w$} |", i, w=max_size)
                    );
                    val.push_str(
                        &format!(" {:>w$} |", xi, w=max_size)
                    );
                }

                format!("{}\n{}\n{}\n", idx, repeat('-').take(idx.len()).join(""), val)
            } else {
                format!("idx | \nval |\n")
            }
    }
}

impl<T: std::fmt::Debug + std::fmt::Display> Debuggable for std::collections::BTreeSet<T> {
    fn debug_string(&self) -> String {
        use itertools::Itertools;
        format!("{{ {} }}", self.iter().join(", "))
    }
}

impl<T, U> Debuggable for std::collections::BTreeMap<T, U> 
where T: std::fmt::Debug + std::fmt::Display, U: std::fmt::Debug + std::fmt::Display
{
    fn debug_string(&self) -> String {
        use itertools::Itertools;
        format!(
            "{{\n{}\n }}", self.iter()
                .map(|(k, v)| format!("{k} -> {v}, "))
                .join("\n")
        )
    }
}

// lg! マクロの定義
#[macro_export]
macro_rules! lg {
    ($val:expr) => {
        #[cfg(feature = "local")]
        {
            {
                use Debuggable;
                let val = &$val;
                eprintln!(
                    "[{}:{}] {} = \n{}",
                    file!(),
                    line!(),
                    stringify!($val),
                    val.debug_string()
                );
                val
            }
        }
    }
}
    