use std::hash::{BuildHasher, Hasher};

type O<T> = Option<T>;  type R<T,E> = Result<T,E>;

macro_rules! bE
  { ($c:expr; $t:expr; $e:expr) => (if $c {$t} else {$e});
    ($c:expr; $t:expr;        ) => (if $c {$t} else {  });
    ($c:expr;        ; $e:expr) => (if $c {  } else {$e}); }
macro_rules! oE
  { ($o:expr; $x:pat,$s:expr; $n:expr)        => (match $o {Some($x)=>$s, None=>$n});
    ($o:expr; $x:pat,$s:expr;        )        => (match $o {Some($x)=>$s, None=>()});
    ($o:expr;               ; $n:expr)        => (match $o {Some(_) =>(), None=>$n}); }
macro_rules! rE
  { ($r:expr; $x:pat,$o:expr; $y:pat,$e:expr) => (match $r {Ok($x)=>$o, Err($y)=>$e});
    ($r:expr; $x:pat,$o:expr;               ) => (match $r {Ok($x)=>$o, Err(_) =>()});
    ($r:expr;               ; $y:pat,$e:expr) => (match $r {Ok(_) =>(), Err($y)=>$e}); }
macro_rules! a {($a:expr) => (assert!($a))}
macro_rules! l {($($x:pat = $v:expr),+) => ($(let $x = $v);+;)}
macro_rules! m {($e:expr) => (&mut $e);  ($e:expr, $r:expr) => (&mut $e[$r])}
macro_rules! r {() => (return);  ($e:expr) => (return $e)}

macro_rules! u8  {($n:expr) => ($n as u8) }  macro_rules! i8  {($n:expr) => ($n as i8) }
macro_rules! u16 {($n:expr) => ($n as u16)}  macro_rules! i16 {($n:expr) => ($n as i16)}
macro_rules! u32 {($n:expr) => ($n as u32)}  macro_rules! i32 {($n:expr) => ($n as i32)}
macro_rules! u64 {($n:expr) => ($n as u64)}  macro_rules! i64 {($n:expr) => ($n as i64)}
macro_rules! uz  {($n:expr) => ($n as uz) }  macro_rules! iz  {($n:expr) => ($n as uz) }

macro_rules! mr
  { ($n:ident, $s:ident; $($a:ident:$t:ty),+; $r:ty $b:block) =>
        (fn $n(&self, $($a:$t),+)-> $r {let $s=self; $b});          }
macro_rules! mm
  { ($($n:ident($s:ident, $($a:ident:$t:ty),*) $r:ty $b:block)+) =>
        ($(fn $n(&mut self, $($a:$t),*)-> $r {let $s=self; $b})+);  }

fn u32b(b: &[u32])-> &[u8]
  { unsafe {std::slice::from_raw_parts(b.as_ptr() as *const _, 4*b.len())}       }
fn u32bm(b: &mut[u32])-> &mut[u8]
  { unsafe {std::slice::from_raw_parts_mut(b.as_mut_ptr() as *mut _, 4*b.len())} }
fn cy<T:Copy>(d: &mut[T], s:&[T])-> uz
  { l![l=d.len().min(s.len())];  d[..l].copy_from_slice(&s[..l]);  l             }

fn cduz(l:uz, r:uz)-> uz {l/r + uz!(l%r != 0)}

const T:bool = true;  const F:bool = false;
#[allow(non_camel_case_types)] type uz = usize;
#[allow(non_camel_case_types)] type iz = isize;

// #[cfg(test)] mod tests {}

#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord)] enum Nk {R=0,S, C,K, O,A,N, V}

pub struct Ar {d:Vec<u32>, h:Box<[u32]>, hl:uz, r: ahash::RandomState}

impl Ar {
mm!{
    al(s, p:uz) (u32, &mut[u32])
      { a!(p<0x10000000);  l![l=s.d.len()];  s.d.resize(l+1+p, 0);  (u32!(l), m![s.d, l..]) }
    rg(s, id:u32, d:&[u32]) ()
      { l![l=s.h.len()];  bE![2*s.hl > l; s.rh(); ];
        let mut h = s.r.build_hasher();  h.write(u32b(d));  let p = uz![h.finish()] % s.h.len();
        for i in 0..l {l![v=m![s.h, (p+i)%l]];  bE![*v==0; {s.hl+=1; r![*v=id+1]}; ]}            }
    rh(s,) () {}
}
}

pub fn mr(a: &mut Ar, s: &[u8], e: &[u8])-> u32
  { a!(s.len()==e.len());  l![l=s.len(), (r,d)=a.al(cduz(2*l, 4))];
    d[0] = u32!(Nk::R) + u32!(l)<<4;  l![d=u32bm(m![d,1..])];  cy(d,s); cy(m![d,l..], e);  r }
pub fn ms(a: &mut Ar, s: &[u8])-> u32
  { l![l=s.len(), (r,d)=a.al(cduz(l,4))];
    d[0] = u32!(Nk::S) + u32!(l)<<4;  l![d=u32bm(m![d,1..])];  cy(d,s);                    r }

fn m1(a: &mut Ar, k:Nk, c:u32)-> u32
  { a!(c < 1<<24);  l![(r,d)=a.al(0)];  d[0] = u32!(k) + u32!(c)<<4;                     r }
fn mm(a: &mut Ar, k:Nk, c:&[u32])-> u32
  { l![l=c.len(), (r,d)=a.al(l)];       d[0] = u32!(k) + u32!(l)<<4;  cy(m![d,1..], c);  r }
