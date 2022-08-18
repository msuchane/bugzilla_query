/*
Copyright 2022 Marek Suchánek <msuchane@redhat.com>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

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
