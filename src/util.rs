use std::path::PathBuf;

pub fn expand_path(input: &str) -> Result<PathBuf, shellexpand::LookupError<std::env::VarError>> {
    let expanded = shellexpand::env(input)?;
    let expanded_tilde = shellexpand::tilde(&expanded);
    Ok(PathBuf::from(expanded_tilde.as_ref()))
}
