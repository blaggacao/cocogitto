use crate::conventional::changelog::release::Release;
use crate::conventional::changelog::template::Template;

use crate::CocoGitto;
use anyhow::anyhow;
use anyhow::Result;

impl CocoGitto {
    /// ## Get a changelog between two oids
    /// - `from` default value:latest tag or else first commit
    /// - `to` default value:`HEAD` or else first commit
    pub fn get_changelog(&self, pattern: &str, _with_child_releases: bool) -> Result<Release> {
        let commit_range = self.repository.revwalk(pattern)?;
        Ok(Release::from(commit_range))
    }

    pub fn get_changelog_at_tag(&self, tag: &str, template: Template) -> Result<String> {
        let changelog = self.get_changelog(tag, false)?;

        changelog
            .into_markdown(template)
            .map_err(|err| anyhow!(err))
    }
}
