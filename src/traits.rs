use std::{ops::{Deref, DerefMut}, thread::panicking};

use crate::Landmine;

impl<T> Deref for Landmine<T>  {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Armed => panic!("Landmine was armed."),
            Self::Safe(inner) => inner
        }
    }
}

impl<T> DerefMut for Landmine<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Armed => panic!("Landmine was armed."),
            Self::Safe(inner) => inner
        }
    }
}

impl<T> Drop for Landmine<T> {
    fn drop(&mut self) {
        match self {
            Self::Armed => {
                if !panicking() {
                    panic!("Landmine was armed.");
                }
            }
            Self::Safe(_) => ()
        }
    }
}