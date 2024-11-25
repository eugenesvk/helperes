use std::ffi::{OsStr,OsString};
/// Concatenate 2 OsStr with a single allocation
pub fn concat_2oss(s1:&OsStr, s2:&OsStr) -> OsString {
  let mut ret = OsString::with_capacity(s1.len() + s2.len()); // allocate once
  ret.push(s1); ret.push(s2); // doesn't allocate
  ret
}
/// Concatenate multiple OsStr with a single allocation
pub fn concat_oss(ss:&[&OsStr]) -> Result<OsString,Box<dyn std::error::Error>> {
  let mut len:usize = 0;
  for s in ss {
    let slen	= s.len();
    let cap 	= usize::MAX - len;
    if slen < cap {
      len += slen;
    } else {return Err(format!("∑ of passed string lengths exceeds usize ‘{}’",usize::MAX).into())}
    }
  let mut ret = OsString::with_capacity(len); // allocate once
  for s in ss { ret.push(s); } // doesn't allocate
  Ok(ret)
}
