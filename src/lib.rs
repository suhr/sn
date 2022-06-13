type O<T> = Option<T>;  type R<T,E> = Result<T,E>;

macro_rules! b {($c:expr; $t:expr; $e:expr)               => (if $c {$t} else {$e})              }
macro_rules! o {($o:expr; $x:pat,$s:expr; $n:expr)        => (match $o {Some($x)=>$s, None=>$n}) }
macro_rules! r {($r:expr; $x:pat,$o:expr; $y:pat,$e:expr) => (match $r {Ok($x)=>$o, Err($y)=>$e})}
macro_rules! a {($a:expr) => (assert!($a))}
macro_rules! ae {($l:expr, $r:expr) => (assert_eq!($l,$r))}
macro_rules! l {($($x:pat,$v:expr),+; $($t:expr);+) => ({$(let $x = $v);+; $($t);+})}

macro_rules! i32 {($n:expr) => ($n as i32)}
macro_rules! u32 {($n:expr) => ($n as u32)}
macro_rules! uz {($n:expr) => ($n as uz)}

macro_rules! mr {
    ($n:ident, $s:ident; $($a:ident:$t:ty),+; $r:ty $b:block) =>
        (fn $n(&self, $($a:$t),+)-> $r {let $s=self; $b});
}
macro_rules! mm {
    ($n:ident, $s:ident; $($a:ident:$t:ty),+; $r:ty $b:block) =>
        (fn $n(&mut self, $($a:$t),+)-> $r {let $s=self; $b});
}

const T:bool = true;  const F:bool = false;
#[allow(non_camel_case_types)] type uz = usize;
#[allow(non_camel_case_types)] type iz = isize;

// #[cfg(test)] mod tests {}

#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord)] enum Nk {R=0,S, C,K, O,A,N, V}

pub struct Ar {d:Vec<u32>, h:Vec<u32>}

impl Ar {
    mm!{al,s; z:u32; (u32, &mut[u32]) {
        l![l,s.d.len(); s.d.resize(l+uz!(z), 0); (u32!(l), &mut s.d[l..])]
    }}
}

pub fn mr(a: &mut Ar, s: &[u8], e: &[u8])-> u32 {
    let l = l![l,s.len();  a!(l==e.len() && l<0x100000);  u32!(l)];
    let (r,d) = a.al((2*l+3)/4 + u32![(2*l+3)%4 != 0]);
    d[0] = u32!(Nk::R) + u32!(l)<<4 + b![l==0; 0; u32!(s[0])]<<24;
    todo!("copy bytes");

    r
}
