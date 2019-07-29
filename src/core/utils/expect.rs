pub struct ExpectList<'a, T> {
    list: Vec<Expect<'a, T>>,
}

impl <'a, T> ExpectList<'a, T> {
    fn before_occur() {

    }
}

pub struct Expect<'a, T> {
    val_type: T,
    expect_type: ExpectType,
    expect_fn: Box<&'a FnOnce()>,
}

impl <'a, T> Expect<'a, T> {
    fn new(val_type: T, expect_type: ExpectType, expect_fn: &'a FnOnce()) -> Expect<T> {
        Expect {
            val_type,
            expect_type,
            expect_fn: Box::new(expect_fn),
        }
    }
    fn occur(&self, val_type: T, expect_type: ExpectType) {
         let is_occur = match self {
            Expect {val_type, expect_type, ..} => true,
            _ => false
        };
        if is_occur {
            let Expect{expect_fn,..} = self;
            expect_fn();
        }
    }
}

pub enum ExpectType {
    Before,
    After,
}
