use anyhow::{bail, Result};
use scraper::node::Element;

pub trait TryAttr {
    fn try_attr(&self, attr: &str) -> Result<&str>;
}

impl TryAttr for Element {
    fn try_attr(&self, attr: &str) -> Result<&str> {
        let Some(value) = self.attr(attr) else {
            bail!("{attr} not found");
        };
        Ok(value)
    }
}
