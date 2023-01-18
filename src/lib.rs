use std::fmt;

/// Type alias for 'u8' that represents a disk in the puzzle
/// The value represents the "size" of the disk
pub type Disk = u8;

/// Struct containing a vector that represents a pole with [disks](Disk) on it
#[derive(Debug, Clone)]
pub struct Pole(Vec<u8>);

impl Pole {
    /// Constructs a new [`Pole`] object
    pub fn new() -> Pole {
        unimplemented!()
    }

    /// Constructs a new [`Pole`] object with ascending [Disks](Disk) stored within
    ///
    /// * `s` - number of disks to create on the pole
    pub fn new_filled(s: u8) -> Pole {
        unimplemented!()
    }

    /// Adds a [`Disk`] (`d`) onto the [`Pole`]
    pub fn push(&mut self, d: Disk) -> Result<(), PoleError> {
        unimplemented!()
    }

    /// Removes the top-most [`Disk`] from the [`Pole`] and returns it
    pub fn pop(&mut self) -> Result<Disk, PoleError> {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq)]
pub enum PoleError {
    PlaceOnSmallerDisk {
        large: u8,
        small: u8,
    },
    RemoveFromEmpty,
}
impl fmt::Display for PoleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}", match *self {
           PoleError::PlaceOnSmallerDisk { large, small } => format!("cannot place disk {} on disk {}", large, small),
           PoleError::RemoveFromEmpty => "cannot remove from empty pole".to_string(),
       })
    }
}
impl std::error::Error for PoleError {}

#[cfg(test)]
mod pole_tests {
    use super::{Pole, PoleError};
    use rand::random;

    #[test]
    fn new_is_empty() {
        let p = Pole::new();
        assert_eq!(p.0.len(), 0);
    }

    #[test]
    fn new_filled_is_filled() {
        let size = random();
        let p = Pole::new_filled(size);

        assert_eq!(p.0.len() as u8, size);
        for i in 0..size {
            assert_eq!(p.0[i as usize], i+1);
        }
    }

    #[test]
    fn push_requires_larger_base_disk_or_empty() {
        let e = Pole::new_filled(1).push(2);
        assert!(e.is_err());
        assert_eq!(e.unwrap_err(), PoleError::PlaceOnSmallerDisk { large: 2, small: 1 });

        let mut p = Pole::new();
        assert!(p.push(2).is_ok());
        assert!(p.push(1).is_ok());
    }

    #[test]
    fn pop_requires_not_empty() {
        let mut p = Pole::new_filled(1);
        assert!(p.pop().is_ok());
        
        let e = p.pop();
        assert!(e.is_err());
        assert_eq!(e.unwrap_err(), PoleError::RemoveFromEmpty);
    }
}
