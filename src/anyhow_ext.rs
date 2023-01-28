use anyhow::{bail, Result};

pub trait IntoAnyhowResult {
    type Target;
    fn into_anyhow_result(self, name: &str) -> Result<Self::Target>;
}

impl<T> IntoAnyhowResult for Option<T> {
    type Target = T;
    fn into_anyhow_result(self, name: &str) -> Result<Self::Target> {
        let Some(v) = self else {
            bail!("unwrap {name} error")
        };
        Ok(v)
    }
}
