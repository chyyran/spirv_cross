#[allow(clippy::cast_ptr_alignment)]
pub fn words_from_bytes(buf: &[u8]) -> &[u32] {
    bytemuck::cast_slice(buf)
}

macro_rules! include_transmute {
    ($file:expr) => {{
        #[repr(C)]
        pub struct AlignedAs<Align, Bytes: ?Sized> {
            pub _align: [Align; 0],
            pub bytes: Bytes,
        }

        static ALIGNED: &AlignedAs::<&[u32], [u8]> = &AlignedAs {
            _align: [],
            bytes: *include_bytes!($file),
        };

        &ALIGNED.bytes
    }};
}

pub(crate) use include_transmute;