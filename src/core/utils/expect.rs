use std::cmp::PartialEq;
pub struct ExpectList<'a, T: PartialEq, U> {
    list: Vec<Expect<'a, T, U>>,
}

impl<'a, T: PartialEq, U> ExpectList<'a,T, U> {
    pub fn new() -> ExpectList<'a, T, U> {
        ExpectList {
            list: Vec::new()
        }
    }
    pub fn add(&mut self, val_type: T,expect_type: ExpectType, expect_fn: Box<FnOnce(U) + 'a>) {
        let expect = Expect::new(val_type,expect_type, expect_fn);
        self.list.push(expect);
    }
    pub fn is_match(&mut self, val_type: &T, expect_type: &ExpectType, expect_val: U) -> bool {
        for (i, item) in self.list.iter().enumerate() {
            if item.is_match(val_type, expect_type) {
                let self_item = self.list.remove(i);
                self_item.occur(expect_val);
                return true
            }
        }
        return false;
    }
}

pub struct Expect<'a, T: PartialEq, U> {
    val_type: T,
    expect_type: ExpectType,
    expect_fn: Box<FnOnce(U) + 'a>,
}

impl<'a, T: PartialEq, U> Expect<'a, T, U> {
    pub fn new(val_type: T, expect_type: ExpectType, expect_fn: Box<FnOnce(U) + 'a>) -> Expect<T, U> {
        Expect {
            val_type,
            expect_type,
            expect_fn: expect_fn,
        }
    }
    pub fn is_match(&self, val_type: &T, expect_type: &ExpectType) -> bool {
        let Expect {
            val_type: self_val_type,
            expect_type: self_expect_type,
            ..
        } = self;
        if self_expect_type == expect_type && val_type == self_val_type {
            return true;
        }

        return false;
    }
    pub fn occur(self, expect_val: U) {
        let Expect { expect_fn, .. } = self;
        expect_fn(expect_val);
    }
}

#[derive(PartialEq)]
pub enum ExpectType {
    Before,
    After,
}
