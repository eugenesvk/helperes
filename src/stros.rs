use std::ffi::{OsStr,OsString};
/// Concatenate 2 OsStr with a single allocation
pub fn concat_2oss<S1:AsRef<OsStr>,S2:AsRef<OsStr>>(s1:S1, s2:S2) -> OsString {
  let s1 = s1.as_ref();
  let s2 = s2.as_ref();
  let mut ret = OsString::with_capacity(s1.len() + s2.len()); // allocate once
  ret.push(s1); ret.push(s2); // doesn't allocate
  ret
}

/// Concatenate multiple OsStr with a single allocation
pub fn concat_oss<S:AsRef<OsStr>>(ss:&[S]) -> Result<OsString,Box<dyn std::error::Error>> {
  let mut len:usize = 0;
  for s in ss {
    let slen	= s.as_ref().len();
    let cap 	= usize::MAX - len;
    if slen < cap {
      len += slen;
    } else {return Err(format!("∑ of passed string lengths exceeds usize ‘{}’",usize::MAX).into())}
    }
  let mut ret = OsString::with_capacity(len); // allocate once
  for s in ss { ret.push(s.as_ref()); } // doesn't allocate
  Ok(ret)
}
