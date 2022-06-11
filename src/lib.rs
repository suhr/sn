type O<T> = Option<T>;  type R<T,E> = Result<T,E>;

macro_rules! b {($c:expr; $t:expr; $e:expr)               => (if $c {$t} else {$e})              }
macro_rules! o {($o:expr; $x:pat,$s:expr; $n:expr)        => (match $o {Some($x)=>$s, None=>$n}) }
macro_rules! r {($r:expr; $x:pat,$o:expr; $y:pat,$e:expr) => (match $r {Ok($x)=>$o, Err($y)=>$e})}

const T:bool = true;  const F:bool = false;
#[allow(non_camel_case_types)] type uz = usize;
#[allow(non_camel_case_types)] type iz = isize;

// #[cfg(test)] mod tests {}

#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord)] enum Nk {R=0,S, C,K, O,A,N}
