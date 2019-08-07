pub struct IntegrationTestSuite {}

impl IntegrationTestSuite {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct InitialState;

mod project_domain;

pub use self::project_domain::*;

impl InitialLandList for InitialState {}
