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

pub fn concat_3oss(s1:S1          ,s2:S2          ,s3:S3                                                                                                          	)
  where            S1:AsRef<OsStr>,S2:AsRef<OsStr>,S3:AsRef<OsStr>                                                                                                	-> Result<OsString> {
  concat_oss_all(&[s1.as_ref()    ,s2.as_ref()    ,s3.as_ref()])                                                                                                  	}
pub fn concat_4oss(s1:S1          ,s2:S2          ,s3:S3          ,s4:S4                                                                                          	)
  where            S1:AsRef<OsStr>,S2:AsRef<OsStr>,S3:AsRef<OsStr>,S4:AsRef<OsStr>                                                                                	-> Result<OsString> {
  concat_oss_all(&[s1.as_ref()    ,s2.as_ref()    ,s3.as_ref()    ,s4.as_ref()])                                                                                  	}
pub fn concat_5oss(s1:S1          ,s2:S2          ,s3:S3          ,s4:S4          ,s5:S5                                                                          	)
  where            S1:AsRef<OsStr>,S2:AsRef<OsStr>,S3:AsRef<OsStr>,S4:AsRef<OsStr>,S5:AsRef<OsStr>                                                                	-> Result<OsString> {
  concat_oss_all(&[s1.as_ref()    ,s2.as_ref()    ,s3.as_ref()    ,s4.as_ref()    ,s5.as_ref()])                                                                  	}
pub fn concat_6oss(s1:S1          ,s2:S2          ,s3:S3          ,s4:S4          ,s5:S5          ,s6:S6                                                          	)
  where            S1:AsRef<OsStr>,S2:AsRef<OsStr>,S3:AsRef<OsStr>,S4:AsRef<OsStr>,S5:AsRef<OsStr>,S6:AsRef<OsStr>                                                	-> Result<OsString> {
  concat_oss_all(&[s1.as_ref()    ,s2.as_ref()    ,s3.as_ref()    ,s4.as_ref()    ,s5.as_ref()    ,s6.as_ref()])                                                  	}
pub fn concat_7oss(s1:S1          ,s2:S2          ,s3:S3          ,s4:S4          ,s5:S5          ,s6:S6          ,s7:S7                                          	)
  where            S1:AsRef<OsStr>,S2:AsRef<OsStr>,S3:AsRef<OsStr>,S4:AsRef<OsStr>,S5:AsRef<OsStr>,S6:AsRef<OsStr>,S7:AsRef<OsStr>                                	-> Result<OsString> {
  concat_oss_all(&[s1.as_ref()    ,s2.as_ref()    ,s3.as_ref()    ,s4.as_ref()    ,s5.as_ref()    ,s6.as_ref()    ,s7.as_ref()])                                  	}
pub fn concat_8oss(s1:S1          ,s2:S2          ,s3:S3          ,s4:S4          ,s5:S5          ,s6:S6          ,s7:S7          ,s8:S8                          	)
  where            S1:AsRef<OsStr>,S2:AsRef<OsStr>,S3:AsRef<OsStr>,S4:AsRef<OsStr>,S5:AsRef<OsStr>,S6:AsRef<OsStr>,S7:AsRef<OsStr>,S8:AsRef<OsStr>                	-> Result<OsString> {
  concat_oss_all(&[s1.as_ref()    ,s2.as_ref()    ,s3.as_ref()    ,s4.as_ref()    ,s5.as_ref()    ,s6.as_ref()    ,s7.as_ref()    ,s8.as_ref()])                  	}
pub fn concat_9oss(s1:S1          ,s2:S2          ,s3:S3          ,s4:S4          ,s5:S5          ,s6:S6          ,s7:S7          ,s8:S8          ,s9:S9          	)
  where            S1:AsRef<OsStr>,S2:AsRef<OsStr>,S3:AsRef<OsStr>,S4:AsRef<OsStr>,S5:AsRef<OsStr>,S6:AsRef<OsStr>,S7:AsRef<OsStr>,S8:AsRef<OsStr>,S9:AsRef<OsStr>	-> Result<OsString> {
  concat_oss_all(&[s1.as_ref()    ,s2.as_ref()    ,s3.as_ref()    ,s4.as_ref()    ,s5.as_ref()    ,s6.as_ref()    ,s7.as_ref()    ,s8.as_ref()    ,s9.as_ref()])  	}
