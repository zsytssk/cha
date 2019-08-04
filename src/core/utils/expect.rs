use std::cmp::PartialEq;
pub struct ExpectList<T: PartialEq, U> {
    list: Vec<Expect<T, U>>,
}

impl<T: PartialEq, U> ExpectList<T, U> {
    pub fn new() -> ExpectList<T, U> {
        ExpectList {
            list: Vec::new()
        }
    }
    pub fn add(&mut self, val_type: T,expect_type: ExpectType,  expect_fn: Box<FnOnce(U)>) {
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

pub struct Expect<T: PartialEq, U> {
    val_type: T,
    expect_type: ExpectType,
    expect_fn: Box<FnOnce(U)>,
}

impl<T: PartialEq, U> Expect<T, U> {
    pub fn new(val_type: T, expect_type: ExpectType, expect_fn: Box<FnOnce(U)>) -> Expect<T, U> {
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
