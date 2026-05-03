// 検証用の書き捨て

fn main() {
    {
        let r;
        {
            let x = 1;
            r = &x;
        }
        assert_eq!(*r, 1);
    }
}
