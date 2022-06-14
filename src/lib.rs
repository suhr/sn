type O<T> = Option<T>;  type R<T,E> = Result<T,E>;

macro_rules! b {($c:expr; $t:expr; $e:expr)               => (if $c {$t} else {$e})              }
macro_rules! o {($o:expr; $x:pat,$s:expr; $n:expr)        => (match $o {Some($x)=>$s, None=>$n}) }
macro_rules! r {($r:expr; $x:pat,$o:expr; $y:pat,$e:expr) => (match $r {Ok($x)=>$o, Err($y)=>$e})}
macro_rules! a {($a:expr) => (assert!($a))}
macro_rules! ae {($l:expr, $r:expr) => (assert_eq!($l,$r))}
macro_rules! l {($($x:pat,$v:expr),+; $($t:expr);+) => ({$(let $x = $v);+; $($t);+})}
macro_rules! m {($e:expr) => (&mut $e)}

macro_rules! u8  {($n:expr) => ($n as u8) }  macro_rules! i8  {($n:expr) => ($n as i8) }
macro_rules! u16 {($n:expr) => ($n as u16)}  macro_rules! i16 {($n:expr) => ($n as i16)}
macro_rules! u32 {($n:expr) => ($n as u32)}  macro_rules! i32 {($n:expr) => ($n as i32)}
macro_rules! u64 {($n:expr) => ($n as u64)}  macro_rules! i64 {($n:expr) => ($n as i64)}
macro_rules! uz  {($n:expr) => ($n as uz) }  macro_rules! iz  {($n:expr) => ($n as uz) }

macro_rules! mr
  { ($n:ident, $s:ident; $($a:ident:$t:ty),+; $r:ty $b:block) =>
        (fn $n(&self, $($a:$t),+)-> $r {let $s=self; $b});          }
macro_rules! mm
  { ($($n:ident($s:ident, $($a:ident:$t:ty),+) $r:ty $b:block)+) =>
        ($(fn $n(&mut self, $($a:$t),+)-> $r {let $s=self; $b})+);  }

fn u32bm(b: &mut[u32])-> &mut[u8]
  { unsafe {std::slice::from_raw_parts_mut(b.as_mut_ptr() as *mut _, 4*b.len())} }
fn cy<T:Copy>(d: &mut[T], s:&[T])-> uz
  { l![l,d.len().min(s.len());  d[..l].copy_from_slice(&s[..l]);  l]             }

fn cduz(l:uz, r:uz)-> uz {l/r + uz!(l%r != 0)}

const T:bool = true;  const F:bool = false;
#[allow(non_camel_case_types)] type uz = usize;
#[allow(non_camel_case_types)] type iz = isize;

// #[cfg(test)] mod tests {}

#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord)] enum Nk {R=0,S, C,K, O,A,N, V}

pub struct Ar {d:Vec<u32>, h:Vec<u32>}

impl Ar {
mm!{
    al(s, z:uz) (u32, &mut[u32]) {l![l,s.d.len();  s.d.resize(l+z, 0);  (u32!(l), m![s.d[l..]])]}
}
}

pub fn mr(a: &mut Ar, s: &[u8], e: &[u8])-> u32
  { let l = s.len();  a!(l==e.len() && l<0x10000000);  let (r,d) = a.al(cduz(2*l+3, 4));
    d[0] = u32!(Nk::R) + u32!(l)<<4;  l![d,u32bm(m!(d[1..])),  l,cy(d,s);  cy(m!(d[l..]),e)];  r }
