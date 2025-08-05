use std::io;
use wallee::{bail, Result};

pub fn bail_literal() -> Result<()> {
    bail!("oh no!");
}

pub fn bail_fmt() -> Result<()> {
    bail!("{} {}!", "oh", "no");
}

pub fn bail_error() -> Result<()> {
    bail!(io::Error::other("oh no!"));
}
