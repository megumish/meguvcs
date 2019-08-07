// project domain
use crate::project::*;
// test utility
use crate::test::*;

pub trait InitialLandList {
    fn get_land_list() -> Vec<Land> {
        unimplemented!()
    }
}

impl HasProjectApp for IntegrationTestSuite {
    type App = Self;

    fn project_app(&self) -> &Self::App {
        self
    }
}

impl ProjectApp for IntegrationTestSuite {}
