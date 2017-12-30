/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

use std::path::Path;

#[fundamental]
pub trait TFileCache: Clone + 'static {
    type File;
    type ResourceUpdates;

    fn add_file<P>(&mut self, P) -> Option<()>
    where
        P: AsRef<Path>;

    fn get_file<P>(&self, P) -> Option<Self::File>
    where
        P: AsRef<Path>;

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates;
}
