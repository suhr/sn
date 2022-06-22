#![allow(unused, non_camel_case_types)]

pub mod m
  { macro_rules! bE
      { ($c:expr; $t:expr; $e:expr)               => (if $c {$t} else {$e});
        ($c:expr; $t:expr;        )               => (if $c {$t} else {  });
        ($c:expr;        ; $e:expr)               => (if $c {  } else {$e});               }
    macro_rules! oE
      { ($o:expr; $x:pat,$s:expr; $n:expr)        => (match $o {Some($x)=>$s, None=>$n});
        ($o:expr; $x:pat,$s:expr;        )        => (match $o {Some($x)=>$s, None=>()});
        ($o:expr;               ; $n:expr)        => (match $o {Some(_) =>(), None=>$n});  }
    macro_rules! rE
      { ($r:expr; $x:pat,$o:expr; $y:pat,$e:expr) => (match $r {Ok($x)=>$o, Err($y)=>$e});
        ($r:expr; $x:pat,$o:expr;               ) => (match $r {Ok($x)=>$o, Err(_) =>()});
        ($r:expr;               ; $y:pat,$e:expr) => (match $r {Ok(_) =>(), Err($y)=>$e}); } 

    macro_rules! a {($a:expr) => (assert!($a))}
    macro_rules! l {($($x:pat = $v:expr),+) => ($(let $x = $v);+;)}
    macro_rules! m {($e:expr) => (&mut $e);  ($e:expr, $r:expr) => (&mut $e[$r])}
    macro_rules! r
      { () => (return);  ($e:expr) => (return $e);
        ($c:expr; $e:expr) => (bE![$c; return $e;]) }

    macro_rules! u8  {($n:expr) => ($n as u8) }  macro_rules! i8  {($n:expr) => ($n as i8) }
    macro_rules! u16 {($n:expr) => ($n as u16)}  macro_rules! i16 {($n:expr) => ($n as i16)}
    macro_rules! u32 {($n:expr) => ($n as u32)}  macro_rules! i32 {($n:expr) => ($n as i32)}
    macro_rules! u64 {($n:expr) => ($n as u64)}  macro_rules! i64 {($n:expr) => ($n as i64)}
    macro_rules! uz  {($n:expr) => ($n as uz) }  macro_rules! iz  {($n:expr) => ($n as uz) }

    pub(crate) use {bE,oE,rE, a,l,m,r, u8,u16,u32,u64,uz, i8,i16,i32,i64,iz};                }

pub type uz = usize;  pub type iz = isize;
pub type O<T> = Option<T>;  pub type R<T,E> = Result<T,E>;
pub const T:bool = true;  pub const F:bool = false;
