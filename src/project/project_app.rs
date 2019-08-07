// project domain
use crate::project::*;

use std::result::Result;

pub trait HasProjectApp {
    type App: ProjectApp;

    fn project_app(&self) -> &Self::App;
}

pub trait ProjectApp {
    fn get_land_list(&self) -> Result<Vec<Land>, crate::error::Error> {
        unimplemented!()
    }
}
