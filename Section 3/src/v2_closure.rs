#[derive(Debug)]
pub struct Hider {
    pub public: String,
    hidden: String,
    hidden_accessed: i32,
}

impl Hider {
    pub fn new(public: String, hidden: String) -> Hider {
        Hider {
            public,
            hidden,
            hidden_accessed: 0,
        }
    }

    pub fn edit<F>(&mut self, f: F)
    where
        F: FnOnce(&mut String),
    {
        f(&mut self.hidden);
        self.hidden_accessed += 1;
    }
}

#[derive(Debug)]
pub struct HideVec {
    v: Vec<String>,
}

impl HideVec {
    pub fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(String::new())
        }
        HideVec { v }
    }

    pub fn edit_all<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut String),
    {
        for item in &mut self.v {
            f(item)
        }
    }
}

fn main() {
    let mut h = Hider::new("showme".to_string(), "hideme".to_string());
    h.edit(|s| s.push_str(" please"));

    println!("Hider = {:?}", h);

    let mut hv = HideVec::new(6);
    let mut count = 0;
    hv.edit_all(|s| {
        s.push_str(&format!("Item = {}", count));
        count += 1;
    });

    println!("HV = {:?}, count = {}", hv, count);
}
