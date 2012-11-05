pub mod uuid {

    enum UUID {
        UUID(u32, u32, u32, u32)
    }

    pub fn random() -> UUID {
        let rng = rand::Rng();
        UUID(
           rng.next(),
           (rng.next() & 0xFFFF0FFF) | 0x00004000, // Force version set to 4
           (rng.next() | 1 << 31) & !(1 << 30), // Set right variant for uuid spec
           rng.next()
       )
    }

    pub fn nil() -> UUID { 
        UUID(0u32, 0u32, 0u32, 0u32)
    }

    impl UUID : ToStr {

        pure fn to_str() -> ~str { 
            let UUID(w1, w2, w3, w4) = self;
            fmt!("%08x-%04x-%04x-%04x-%04x%08x", 
                 w1               as uint,
                (w2 >> 16)        as uint,
                (w2 & 0x0000FFFF) as uint,
                (w3 >> 16)        as uint,
                (w3 & 0x0000FFFF) as uint,
                 w4               as uint)
        }

    }

    impl UUID : cmp::Eq {

        pure fn eq(other: &UUID) -> bool {
            let UUID(sw1, sw2, sw3, sw4) = self;
            let UUID(ow1, ow2, ow3, ow4) = *other;

            sw1 == ow1 && 
            sw2 == ow2 && 
            sw3 == ow3 && 
            sw4 == ow4
        }

        pure fn ne(other: &UUID) -> bool {
            !self.eq(other)
        }

    }

}

#[cfg(test)]
mod test {

    #[test]
    fn gen_test() {
        uuid::random();
    }

    #[test]
    fn eq_test() {
        let id = uuid::random();
        assert id == id;
        assert id != uuid::random();
    }

}
