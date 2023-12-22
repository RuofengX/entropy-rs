use std::{
    collections::HashMap,
    thread::{self, JoinHandle},
};

use rayon::prelude::*;
use sled::{Config, Db};
