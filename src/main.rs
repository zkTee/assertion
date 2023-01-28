mod assertion;
use assertion::*;
mod a;
use a::*;
mod b;
use b::*;

fn main() {
    let use_a = false;
	if use_a {
		a();

		return;
	}

	b();
}
