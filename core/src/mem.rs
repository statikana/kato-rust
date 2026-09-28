
// Similar to a pointer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Handle {
    pub slot: u32, // 4 bytes
    pub g: u32, // generation counter of the slot this handle is at
    pub len: u32, // 4 bytes
}

pub enum DynTag {
    Null,
    String,
    Array(Box<DynTag>), // element tag
    Matrix(Box<DynTag>, Vec<u32>) // element tag, shape
}

impl Default for DynTag {
    fn default() -> Self { Self::Null }
}

pub struct Blob {
    pub tag: DynTag,
    // Vec, not Box<[u8]>: appending is a push, but the buffer moves on realloc, so a pointer
    // into it is only good until the next growth. Box it if callers need stable addresses.
    pub bytes: Vec<u8>
}

// owner of assigned memory (blobs)
pub struct Arena {
    blobs: Vec<Blob>,
    gens: Vec<u32>,
    free: Vec<u32>
}

impl Arena {
    pub fn new() -> Self {
        Arena { blobs: Vec::new(), gens: Vec::new(), free: Vec::new() }
    }

    pub fn alloc(&mut self, tag: DynTag, bytes: Vec<u8>) -> Handle {
        let len = u32::try_from(bytes.len()).expect("blob expects u32::MAX bytes");
        let blob = Blob { tag, bytes };

        match self.free.pop() { // next free location?
            Some(slot) => {
                let g = self.gens[slot as usize];
                self.blobs[slot as usize] = blob;
                Handle { slot, g, len }
            },
            None => {
                let slot = u32::try_from(self.blobs.len()).expect("arena full (u32)");
                self.blobs.push(blob);
                self.gens.push(0);
                Handle { slot, g: 0, len }
            }
        }
    }

    pub fn get(&self, h: Handle) -> Option<&Blob> {
        let i = h.slot as usize;
        let g = *self.gens.get(i)?;
        if g != h.g { return None }
        let blob = self.blobs.get(i)?;
        debug_assert_eq!(blob.bytes.len(), h.len as usize, "handle length disagrees with blob");
        Some(blob)
    }

    pub fn get_mut(&mut self, h: Handle) -> Option<&mut Blob> {
        let i = h.slot as usize;
        let g = *self.gens.get(i)?;
        if g != h.g { return None }
        let blob = self.blobs.get_mut(i)?;
        debug_assert_eq!(blob.bytes.len(), h.len as usize, "handle length disagrees with blob");
        Some(blob)
    }

    pub fn free(&mut self, h: Handle) {
        let i = h.slot as usize;
        match self.gens.get(i) {
            Some(&g) if g == h.g => {}
            _ => return,
        }
        // keeps handles from different lifetimes of this slot apart; does NOT stay unique
        // past u32::MAX frees, at which point a very old handle looks live again
        self.gens[i] = self.gens[i].wrapping_add(1);
        self.blobs[i] = Blob {tag: Default::default(), bytes: Vec::new() };
        self.free.push(h.slot);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_is_three_words() {
        assert_eq!(std::mem::size_of::<Handle>(), 12);
    }

    #[test]
    fn alloc_hands_back_the_bytes_it_was_given() {
        let mut arena = Arena::new();
        let h = arena.alloc(DynTag::String, vec![1, 2, 3]);
        assert_eq!(h.len, 3);
        assert_eq!(arena.get(h).unwrap().bytes, vec![1, 2, 3]);
        assert!(matches!(arena.get(h).unwrap().tag, DynTag::String));
    }

    #[test]
    fn free_invalidates_the_handle() {
        let mut arena = Arena::new();
        let h = arena.alloc(DynTag::String, vec![1]);
        arena.free(h);
        assert!(arena.get(h).is_none());
    }

    #[test]
    fn freed_slot_is_reused_but_the_old_handle_stays_dead() {
        let mut arena = Arena::new();
        let old = arena.alloc(DynTag::String, vec![1, 2, 3]);
        arena.free(old);

        let new = arena.alloc(DynTag::String, vec![9]);
        assert_eq!(new.slot, old.slot, "slot should have been reused");
        assert_ne!(new.g, old.g, "generations must differ");
        assert!(arena.get(old).is_none(), "stale handle aliased the new blob");
        assert_eq!(arena.get(new).unwrap().bytes, vec![9]);
    }

    #[test]
    fn out_of_range_slot_is_none() {
        let mut arena = Arena::new();
        arena.alloc(DynTag::String, vec![1]);
        let bogus = Handle { slot: 7, g: 0, len: 0 };
        assert!(arena.get(bogus).is_none());
    }

    #[test]
    fn freeing_twice_does_not_double_push_the_slot() {
        let mut arena = Arena::new();
        let h = arena.alloc(DynTag::String, vec![1]);
        arena.free(h);
        arena.free(h); // second free is ignored, the generation no longer matches
        let next = arena.alloc(DynTag::String, vec![2]);
        assert_eq!(next.slot, h.slot);
    }
}
