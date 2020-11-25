//! Command `tag`
use crate::{registry::Registry, result::Result};

/// Exec comamnd `tag`
pub fn exec(registry: Registry, limit: usize, update: bool) -> Result<()> {
    if update {
        println!("Fetching registry...");
        registry.update()?;
    }

    // Get tags
    let mut tags = registry.tag()?;
    if tags.is_empty() {
        registry.update()?;
    }

    let last = if limit < tags.len() || limit < 1 {
        limit
    } else {
        tags.len()
    };

    tags.reverse();
    println!("{}", &tags[..last].join("\n"));
    Ok(())
}
