use std::fmt;

/// Type alias for 'u8' that represents a disk in the puzzle
/// The value represents the "size" of the disk
pub type Disk = u8;

/// Struct containing a vector that represents a pole with [disks](Disk) on it
#[derive(Debug, Clone)]
pub struct Pole(Vec<u8>);

impl Pole {
    /// Constructs a new [`Pole`] object
    fn new() -> Pole {
        unimplemented!()
    }

    /// Constructs a new [`Pole`] object with ascending [Disks](Disk) stored within
    fn new_filled() -> Pole {
        unimplemented!()
    }

    /// Adds a [`Disk`] onto the [`Pole`]
    fn push(d: Disk) -> Result<(), PoleError> {
        unimplemented!()
    }

    /// Removes the top-most [`Disk`] from the [`Pole`] and returns it
    fn pop() -> Result<Disk, PoleError> {
        unimplemented!()
    }
}

#[derive(Debug)]
enum PoleError {
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
