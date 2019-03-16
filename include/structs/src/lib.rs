use hlvm::function::*;


pub fn strukt(v: Value) -> Value {
    let mut o = empty_obj();

    let mut i = 0;
    let l = v.get_list();
    let len = l.len();
    loop {
        if i >= len {
            break;
        }

        o.set_attr(
            l[i].as_string(), l[i+1].clone()
        );

        i += 2;
    }

    o
}