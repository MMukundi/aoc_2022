#[derive(Debug,Default,Clone,PartialEq, Eq)]
pub struct Unzip<A,B>{
    a:A,
    b:B,
}
impl <A:Extend<AItem>,B:Extend<BItem>,AItem,BItem> Extend<(AItem,BItem)> for Unzip<A,B>{
    fn extend<T: IntoIterator<Item = (AItem,BItem)>>(&mut self, iter: T) {
        for (a,b) in iter {
            self.a.extend(Some(a));
            self.b.extend(Some(b));
        }
    }
    #[cfg(feature="reserve_one")]
    #[inline]
    fn extend_reserve(&mut self, additional: usize) {
        self.a.extend_reserve(additional);
        self.b.extend_reserve(additional);
    }
    #[cfg(feature="reserve_one")]
    #[inline]
    fn extend_one(&mut self, (a,b): (AItem,BItem)) {
        self.a.extend_one(a);
        self.b.extend_one(b);
    }
}
