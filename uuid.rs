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

}



#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let id = uuid::random();
        io::println(id.to_str());
    }

}
