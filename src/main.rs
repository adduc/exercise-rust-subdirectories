mod tier1;

use tier1::tier1a::call;
use tier1::tier2::tier2a::call as call2;



fn main() {
    println!("Hello, world!");

    call();
    call2();
}
