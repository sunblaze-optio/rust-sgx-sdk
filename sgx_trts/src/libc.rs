// Copyright (c) 2017 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! This mod provides the interface connecting Rust's memory management system
//! to the Intel's SGX SDK's malloc system.
//!
//! It is a c-style interface and self-explained. Currently we don't have much
//! time for documenting it.
use sgx_types::*;

#[link(name = "sgx_tstdc")]
extern {

    //pub fn memchr(s: * const c_void, c: c_int, n: size_t) -> *mut c_void;
    //pub fn memrchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn strlen(s: * const c_char) -> size_t;
    pub fn calloc(nobj: size_t, size: size_t) -> * mut c_void;
    pub fn malloc(size: size_t) -> * mut c_void;
    pub fn realloc(p: * mut c_void, size: size_t) -> * mut c_void;
    pub fn free(p: * mut c_void);
    pub fn posix_memalign(memptr: * mut * mut c_void, align: size_t, size: size_t) -> c_int;
}

pub unsafe fn memchr(s: * const u8, c: u8, n: usize) -> * const u8 {

    let mut ret = 0 as * const u8;
    let mut p = s;

    for _ in 0..n {
        if *p == c {
            ret = p;
            break;
        }
        p = p.offset(1);
    }
    ret
}

pub unsafe fn memrchr(s: * const u8, c: u8, n: usize) -> * const u8 {

    if n == 0 {return 0 as * const u8}

    let mut ret = 0 as * const u8;
    let mut p: * const u8 = (s as usize + (n - 1)) as * const u8;
    for _ in 0..n {
        if *p == c {
            ret = p;
            break;
        }
         p = p.offset(-1);
    }
    ret
}
