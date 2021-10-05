mod nation {
    pub mod government {
        pub fn govern() {}
    }

    pub mod congress {
        pub fn legislate() -> i32 {
            10
        }
    }
    pub mod court {
        pub fn judicial() {
            super::congress::legislate();
        }
    }
}

fn main() {
    use nation::government;
    government::govern();

    let num = nation::congress::legislate();
    println!("num:{}", num);
    nation::court::judicial();
    println!("--------------")
}
