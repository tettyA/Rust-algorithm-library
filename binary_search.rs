pub trait BinarySearch<T> {
    fn lower_bound(&self, key: T) -> usize;
    fn upper_bound(&self, key: T) -> usize;
}
impl<T> BinarySearch<T> for Vec<T>
where
    T: Ord,
{
    fn lower_bound(&self, key: T) -> usize {
        //ng...条件を満たさない中で最小の番地。ok...条件を満たす中で最大の番地
        let mut ng: isize = -1;
        let mut ok: isize = self.len() as isize;

        while ok - ng > 1 {
            let mid: isize = (ok + ng) / 2;
            if self[mid as usize] >= key {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
    fn upper_bound(&self, key: T) -> usize {
        let mut ng: isize = -1;
        let mut ok: isize = self.len() as isize;

        while ok - ng > 1 {
            let mid: isize = (ok + ng) / 2;
            if self[mid as usize] > key {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
}
