//! test of getting land list

// test utility
use crate::test::*;
// project domain
use crate::project::*;

#[test]
fn it_works() -> Result<(), crate::error::Error> {
    let suite = IntegrationTestSuite::new();

    let land_list = suite.project_app().get_land_list()?;

    assert_eq!(land_list, InitialState::get_land_list());

    Ok(())
}
