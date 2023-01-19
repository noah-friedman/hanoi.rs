use std::fmt;

/// Type alias for 'u8' that represents a disk in the puzzle
/// The value represents the "size" of the disk
pub type Disk = u8;

/// Struct containing a vector that represents a pole with [disks](Disk) on it
#[derive(Debug, Clone)]
pub struct Pole(Vec<Disk>);

impl Pole {
    /// Constructs a new [`Pole`] object
    pub const fn new() -> Pole {
        Pole(Vec::new())
    }

    /// Deletes the current contents and fills the [`Pole`] with ascending [Disks](Disk) stored within
    ///
    /// Returns [&mut self](Pole) for chaining
    ///
    /// * `s` - number of disks to create on the pole
    pub fn fill(&mut self, s: u8) {
        // Create a vector
        let mut v = Vec::new();
        
        // Fill it up with descending values
        for i in (1..=s).rev() {
            v.push(i);
        }

        // Replace the Pole data with the new vector
        self.0 = v;
    }

    /// Adds a [`Disk`] (`d`) onto the [`Pole`]
    pub fn push(&mut self, d: Disk) -> Result<(), PoleError> {
        // Verify that the disk is smaller than the top disk on the Pole...
        return if self.0.len() > 0 && self.0[0] < d { 
            // ... if not, return an error ...
            Err(PoleError::PlaceOnSmallerDisk { large: d, small: self.0[0] }) 
        } else {
            // ... otherwise, push the Disk onto the Pole and return
            self.0.push(d);
            Ok(())
        }
    }

    /// Removes the top-most [`Disk`] from the [`Pole`] and returns it
    pub fn pop(&mut self) -> Result<Disk, PoleError> {
        // Attempt to pop a value from data vector ...
        match self.0.pop() {
            Some(v) => Ok(v),   // ... return it if everything is okay ...
            None => Err(PoleError::RemoveFromEmpty) // ... throw an error if there is no value
        }
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
    fn fill_fills_pole() {
        let size = random();

        let mut p = Pole::new();
        p.fill(size);

        assert_eq!(p.0.len() as u8, size);
        for i in 0..size {
            assert_eq!(p.0[i as usize], size-i);
        }
    }

    #[test]
    fn push_requires_larger_base_disk_or_empty() {
        let mut p = Pole::new();
        p.fill(1);
        let e = p.push(2);
        assert!(e.is_err());
        assert_eq!(e.unwrap_err(), PoleError::PlaceOnSmallerDisk { large: 2, small: 1 });

        let mut p = Pole::new();
        assert!(p.push(2).is_ok());
        assert!(p.push(1).is_ok());
    }

    #[test]
    fn pop_requires_not_empty() {
        let mut p = Pole::new();
        p.fill(1);
        assert!(p.pop().is_ok());
        let e = p.pop();
        assert!(e.is_err());
        assert_eq!(e.unwrap_err(), PoleError::RemoveFromEmpty);
    }
}
