use std::{mem::ManuallyDrop, ptr::read};

use crate::Landmine;

impl<T> Landmine<T> {
    /// Converts `Landmine<T>` into a "safe" `Option<T>`.
    pub fn defuse(self) -> Option<T> {
        let landmine = ManuallyDrop::new(self);
        
        match &*landmine {
            Self::Armed => None,
            Self::Safe(inner) => Some(
                // SAFETY: The inner T has effectively transferred ownership
                unsafe {
                    read(inner)
                }
            )
        }
    }

    /// Consumes the `Landmine` wrapper and returns the contained `T`.
    pub fn unwrap(self) -> T {
        let landmine = ManuallyDrop::new(self);

        match &*landmine {
            Self::Armed => panic!("Landmine was armed."),
            Self::Safe(inner) => {
                // SAFETY: The inner T has effectively transferred ownership
                unsafe {
                    read(inner)
                }
            }
        }
    }
}