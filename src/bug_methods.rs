use crate::bug_model::Bug;

impl Bug {
    /// Returns a the value of the flag corresponding to the flag name.
    /// If no flag by that name is set in the bug, the function returns None.
    ///
    /// # Panics
    ///
    /// The function panics if flags are not available at all.
    /// Enable flags when accessing the Bugzilla instance.
    #[must_use]
    pub fn get_flag<'a>(&'a self, name: &str) -> Option<&'a str> {
        let flags = self
            .flags
            .as_ref()
            .expect("The bug has no flags. Enable flags when accessing Bugzilla.");
        let flag = flags.iter().find(|f| f.name == name)?;
        Some(&flag.status)
    }
}
