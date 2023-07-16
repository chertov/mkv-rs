#[macro_use] extern crate anyhow;
#[macro_use] extern crate log;

mod gen;
mod io;
mod errors;

pub mod element;

pub use errors::MatroskaError;

use async_trait::async_trait;
pub use gen::*;

use gen::ids::EbmlId;

#[async_trait::async_trait]
pub trait ElementRead<R: tokio::io::AsyncRead + Send + Unpin> {
    async fn read(r: &mut R) -> Result<(Self, usize), anyhow::Error> where Self: Sized;
    async fn read_header(r: &mut R) -> Result<(ElementSize, usize), anyhow::Error>;
    async fn read_body(r: &mut R, size: ElementSize) -> Result<(Self, usize), anyhow::Error> where Self: Sized;
}
#[async_trait::async_trait]
pub trait ElementWrite<W: tokio::io::AsyncWrite + Send + Unpin> {
    async fn write(&self, w: &mut W) -> Result<usize, anyhow::Error>;
    async fn write_header(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error>;
    async fn write_body(&self, w: &mut W) -> Result<usize, anyhow::Error>;
}

pub trait ElementReadBlocking<R: std::io::Read> {
    fn read(r: &mut R) -> Result<(Self, usize), anyhow::Error> where Self: Sized;
    fn read_header(r: &mut R) -> Result<(ElementSize, usize), anyhow::Error>;
    fn read_body(r: &mut R, size: ElementSize) -> Result<(Self, usize), anyhow::Error> where Self: Sized;
}
pub trait ElementWriteBlocking<W: std::io::Write> {
    fn write_blocking(&self, w: &mut W) -> Result<usize, anyhow::Error>;
    fn write_header_blocking(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error>;
    fn write_body_blocking(&self, w: &mut W) -> Result<usize, anyhow::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ElementSize {
    Sized(u64),
    Unknown(u64),
}
impl ElementSize {
    pub fn unwrap(&self) -> u64 {
        match self {
            ElementSize::Sized(val) => *val,
            ElementSize::Unknown(val) => *val
        }
    }
    pub fn try_sized(&self, id: EbmlId) -> Result<u64, anyhow::Error> {
        match self {
            ElementSize::Sized(size) => Ok(*size),
            ElementSize::Unknown(val) =>
                Err(anyhow::anyhow!("Element ID '{:?}' unknown data size is not allowed", id))
        }
    }
}
impl Default for ElementSize {
    fn default() -> Self {
        ElementSize::Unknown(io::SIZE_UNKNOWN)
    }
}


#[derive(Debug,Eq,PartialEq,Clone,Copy)]
pub enum ElementType {
    UInteger,
    Integer,
    Float,
    String,
    Utf8,
    Binary,
    Date,
    Struct,
}

#[derive(Clone)]
pub struct Ebml<T> {
    pub index: Option<u64>,
    pub v: Box<T>,
    id: u64,
}
impl<T> Ebml<T> {
    pub fn new(v: T) -> Self {
        Self { index: None, v: Box::new(v), id: Self::rnd_id() }
    }
    pub fn new_index(index: u64, v: T) -> Self {
        Self { index: Some(index), v: Box::new(v), id: Self::rnd_id() }
    }
    fn rnd_id() -> u64 { use rand::Rng; rand::thread_rng().gen_range(u32::MAX as u64 .. u64::MAX) }
    // fn rnd_id() -> u64 { use rand::Rng; rand::thread_rng().gen_range(1000 .. 9999) }
}
impl<T: Default> Default for Ebml<T> {
    fn default() -> Self {
        Self {
            index: None,
            v: Box::new(T::default()),
            id: Self::rnd_id(),
        }
    }
}
impl<T: std::fmt::Debug> std::fmt::Debug for Ebml<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            let index = match self.index {
                Some(index) => format!("{index}"),
                None => "None".to_string(),
            };
            write!(f, "Ebml(index:{index}, {:#?})", self.v)
        } else {
            self.v.fmt(f)
        }
    }
}

#[macro_export]
macro_rules! impl_ord {
    ($type_name:ident) => {
        impl PartialEq<Self> for $type_name {
            fn eq(&self, other: &Self) -> bool {
                let (_, _, self_id) = self.index();
                let (_, _, other_id) = other.index();
                self_id.eq(&other_id)
            }
        }
        impl Eq for $type_name {}

        impl PartialOrd for $type_name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                let (_, self_index, self_id) = self.index();
                let (_, other_index, other_id) = other.index();
                if let Some(index) = self_index {
                    if let Some(other_index) = other_index{
                        match index.partial_cmp(&other_index) {
                            Some(std::cmp::Ordering::Equal) => {}
                            None => {},
                            res => return res,
                        }
                    }
                }
                self_id.partial_cmp(&other_id)
            }
        }
        impl Ord for $type_name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                let (_, self_index, self_id) = self.index();
                let (_, other_index, other_id) = other.index();
                if let Some(index) = self_index {
                    if let Some(other_index) = other_index {
                        match index.cmp(&other_index) {
                            std::cmp::Ordering::Equal => {}
                            res => return res,
                        }
                    }
                }
                self_id.cmp(&other_id)
            }
        }
    }
}
