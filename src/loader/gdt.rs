use kvm_bindings::kvm_segment;

pub struct Entry {
    // TODO: Reduce duplication.
    base: u32,
    limit: u32,

    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    gran: u8,
    base_high: u8,
}

impl Entry {
    pub fn new(base: u32, limit: u32, access: u8, gran: u8) -> Entry {
        Entry {
            base,
            limit,

            limit_low: limit as u16 & 0xffff,
            base_low: base as u16 & 0xffff,
            base_middle: (base >> 16) as u8 & 0xff,
            access: access,
            gran: gran & 0xf0,
            base_high: (base >> 24) as u8 & 0xff,
        }
    }

    pub fn pack(&self) -> u64 {
        (u64::from(self.base_high) << 56)
            | (u64::from(self.gran) << 48)
            | (u64::from(self.access) << 40)
            | (u64::from(self.base_middle) << 32)
            | (u64::from(self.base_low) << 16)
            | (u64::from(self.limit_low))
    }

    pub fn segment(&self) -> kvm_segment {
        kvm_segment {
            base: self.base as u64,
            limit: self.limit,
            selector: 0,
            type_: self.get_type(),
            present: 0,
            dpl: self.get_dpl(),
            db: self.get_d(), // TODO: Is this right?
            s: 0,             // ?
            l: 0,             // ?
            g: self.get_g(),
            avl: 0, // ?
            unusable: 0,
            padding: 0,
        }
    }

    /// Is segment present?
    fn get_p(&self) -> u8 {
        self.access >> 7
    }

    /// Get descriptor privilege level (ring 0-3).
    fn get_dpl(&self) -> u8 {
        self.access & 0b0110_0000 >> 5
    }

    /// Descriptor type.
    fn get_dt(&self) -> u8 {
        self.access & 0b0001_0000 >> 4
    }

    /// Segment type.
    fn get_type(&self) -> u8 {
        self.access & 0b0000_1111
    }

    /// Granularity (0 = 1 byte, 1 = 1kbyte).
    fn get_g(&self) -> u8 {
        self.gran >> 7
    }

    /// Operand size (0 = 16bit, 1 = 32bit).
    fn get_d(&self) -> u8 {
        self.gran & 0b0100_0000 >> 6
    }
}
