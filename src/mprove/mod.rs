/*

Copyright 2020 by Suyash Bagad, Saravanan Vijayakumaran

This file is part of MProve-Ristretto library
Link: https://github.com/suyash67/MProve-Ristretto

*/

pub mod mprove_sigs;
pub mod mprove;

#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum Errors {
    MProveSigsError,
    MProveError,
}
