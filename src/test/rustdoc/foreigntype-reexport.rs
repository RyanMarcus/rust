// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(extern_types)]

mod sub {
    extern {
        /// Another extern type.
        pub type C2;
        pub fn f2();
        pub static K: usize;
    }
}

pub mod sub2 {
    extern {
        // @has foreigntype_reexport/sub2/foreigntype.C.html
        pub type C;
        // @has foreigntype_reexport/sub2/fn.f.html
        pub fn f();
        // @has foreigntype_reexport/sub2/static.K3.html
        pub static K3: usize;
    }
}

mod sub3 {
    extern {
        pub type C4;
        pub fn f4();
        pub static K4: usize;
        type X4;
    }
}

// @has foreigntype_reexport/foreigntype.C2.html
// @has foreigntype_reexport/fn.f2.html
// @has foreigntype_reexport/static.K2.html
// @has foreigntype_reexport/index.html '//a[@class="foreigntype"]' 'C2'
// @has foreigntype_reexport/index.html '//a[@class="fn"]' 'f2'
// @has foreigntype_reexport/index.html '//a[@class="static"]' 'K2'
pub use self::sub::{C2, f2, K as K2};

// @has foreigntype_reexport/index.html '//a[@class="foreigntype"]' 'C'
// @has foreigntype_reexport/index.html '//a[@class="fn"]' 'f'
// @has foreigntype_reexport/index.html '//a[@class="static"]' 'K3'
// @has foreigntype_reexport/index.html '//code' 'pub use self::sub2::C as C3;'
// @has foreigntype_reexport/index.html '//code' 'pub use self::sub2::f as f3;'
// @has foreigntype_reexport/index.html '//code' 'pub use self::sub2::K3;'
pub use self::sub2::{C as C3, f as f3, K3};

// @has foreigntype_reexport/foreigntype.C4.html
// @has foreigntype_reexport/fn.f4.html
// @has foreigntype_reexport/static.K4.html
// @!has foreigntype_reexport/foreigntype.X4.html
// @has foreigntype_reexport/index.html '//a[@class="foreigntype"]' 'C4'
// @has foreigntype_reexport/index.html '//a[@class="fn"]' 'f4'
// @has foreigntype_reexport/index.html '//a[@class="static"]' 'K4'
// @!has foreigntype_reexport/index.html '//a[@class="foreigntype"]' 'X4'
pub use self::sub3::*;
