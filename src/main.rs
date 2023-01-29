mod assertion;
use assertion::*;
mod a;
use a::*;
mod b;
use b::*;
mod c;
use c::*;

fn main() {
	a();

    b();

    c();
}
