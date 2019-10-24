use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct WithLife<'a> {
    s: &'a String,
}

#[derive(Debug)]
pub struct NoLife {
    s: Rc<RefCell<String>>,
}

fn main() -> Result<(), std::io::Error> {
    //let (l1, l2) = make_with_life("test_data/v3_data.txt")?;
    let (n1, n2) = make_no_life("test_data/v3_data.txt")?;

    let mut s = n1.s.borrow_mut();

    //let s2 = n2.s.borrow();
    //println!("s2 = {}", s2);
    s.push_str("What if it was even bigger");

    println!("n1 = {:?}", n1);
    println!("n2 = {:?}", n2);

    println!("s == {}", s);
    drop(s);

    println!("n1 = {:?}", n1);
    println!("n2 = {:?}", n2);
    Ok(())
}

/*
fn make_with_life<'a>(fname: &str) -> Result<(WithLife<'a>, WithLife<'a>), std::io::Error> {
    let s = std::fs::read_to_string(fname)?;
    Ok((WithLife { s: &s }, WithLife { s: &s }))
}
*/

fn make_no_life(fname: &str) -> Result<(NoLife, NoLife), std::io::Error> {
    let s = std::fs::read_to_string(fname)?;
    let r = Rc::new(RefCell::new(s));
    Ok((NoLife { s: r.clone() }, NoLife { s: r }))
}
