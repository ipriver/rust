// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// The type of `y` ends up getting inferred to the type of the block.
fn broken() {
    let mut x = 3;
    let mut _y = vec!(&mut x);
    //~^ NOTE borrow of `x` occurs here
    //~| NOTE borrow of `x` occurs here
    //~| NOTE borrow of `x` occurs here
    while x < 10 { //~ ERROR cannot use `x` because it was mutably borrowed
        //~^ NOTE use of borrowed `x`
        let mut z = x; //~ ERROR cannot use `x` because it was mutably borrowed
        //~^ NOTE use of borrowed `x`
        _y.push(&mut z); //~ ERROR `z` does not live long enough
        //~^ NOTE does not live long enough
        x += 1; //~ ERROR cannot assign
        //~^ NOTE assignment to borrowed `x` occurs here
    }
    //~^ NOTE borrowed value only lives until here
}
//~^ NOTE borrowed value needs to live until here

fn main() { }
