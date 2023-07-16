use anyhow::Context;

use time::OffsetDateTime;
use super::gen::ids::EbmlId;
use super::{ io, MatroskaError, ElementSize, ElementType };

#[derive(Debug)]
pub enum ElementContent {
    UInteger(u64),
    Integer(i64),
    Float(f64),
    String(String),
    Utf8(String),
    Binary(Vec<u8>),
    Date(OffsetDateTime),
    Struct(Vec<Element>),
}

// #[async_trait::async_trait]
// pub trait ElementRead<R: tokio::io::AsyncRead + Send + Unpin> {
//     async fn read(r: &mut R) -> Result<(Self, usize), anyhow::Error> where Self: Sized;
//     async fn read_header(r: &mut R) -> Result<(ElementSize, usize), anyhow::Error>;
//     async fn read_body(r: &mut R, size: ElementSize) -> Result<(Self, usize), anyhow::Error> where Self: Sized;
// }
// pub trait ElementReadBlocking<R: std::io::Read> {
//     fn read(r: &mut R) -> Result<(Self, usize), anyhow::Error> where Self: Sized;
//     fn read_header(r: &mut R) -> Result<(ElementSize, usize), anyhow::Error>;
//     fn read_body(r: &mut R, size: ElementSize) -> Result<(Self, usize), anyhow::Error> where Self: Sized;
// }


#[derive(Debug)]
pub struct Element {
    pub id: EbmlId,
    pub type_: ElementType,
    pub header_len: u64,
    pub size: ElementSize,
}
impl Element {
    pub fn write_header_blocking<W: std::io::Write>(w: &mut W, id: EbmlId, size: u64) -> Result<(), anyhow::Error> {
        io::blocking::write_element_id_size(w, id as u64, size)?;
        Ok(())
    }
    pub fn read_header_blocking<R: std::io::Read>(r: &mut R) -> Result<Element, anyhow::Error> {
        let (id, size, header_len) = super::io::blocking::read_element_id_size(r)
            .context(format!("Element::parse, failed to read_element_id_size()"))?;
        Ok(Element { id, type_: id.type_(), header_len, size })
    }

    pub fn skip_blocking<R: std::io::Seek>(&self, r: &mut R) -> Result<(), anyhow::Error> {
        // if self.type_ == ElementType::Struct { return Ok(()) }
        r.seek(std::io::SeekFrom::Current(self.size.unwrap() as i64)).context(format!("Failed to read_exact(r, {:?})", self.size))?;
        Ok(())
    }

    pub fn read_body_blocking<R: std::io::Read>(&self, r: &mut R) -> Result<ElementContent, anyhow::Error> {
        let size = self.size;
        Ok(match self.id.type_() {
            ElementType::UInteger => io::blocking::read_uint(r, size.unwrap()).map(|val| ElementContent::UInteger(val))?,
            ElementType::Integer => io::blocking::read_int(r, size.unwrap()).map(|val| ElementContent::Integer(val))?,
            ElementType::Float => io::blocking::read_float(r, size.unwrap()).map(|val| ElementContent::Float(val))?,
            ElementType::String => io::blocking::read_string(r, size.unwrap()).map(|val| ElementContent::String(val))?,
            ElementType::Utf8 => io::blocking::read_utf8(r, size.unwrap()).map(|val| ElementContent::Utf8(val))?,
            ElementType::Binary => io::blocking::read_bin(r, size.unwrap()).map(|val| ElementContent::Binary(val))?,
            ElementType::Date => io::blocking::read_date(r, size.unwrap()).map(|val| ElementContent::Date(val))?,

            ElementType::Struct => Self::read_struct_blocking(r, size).map(|val| ElementContent::Struct(val))?,
        })
    }

    pub fn read_struct_blocking<R: std::io::Read>(r: &mut R, mut size: ElementSize) -> Result<Vec<Element>, anyhow::Error> {
        let mut size = size.unwrap();
        let mut elements = Vec::new();
        while size > 0 {
            let e = Element::read_header_blocking(r).context(format!("Element::parse_struct, failed to read_element_id_size()"))?;
            if e.size.unwrap() > size {
                Err(MatroskaError::InvalidSize(e.id, e.size))?;
            }
            size -= e.size.unwrap();
            elements.push(e);
        }
        Ok(elements)
    }
}

impl Element {
    // pub fn write_header<W: std::io::Write>(w: &mut W, id: EbmlId, size: u64) -> Result<(), anyhow::Error> {
    //     io::write_element_id_size(w, id as u64, size)?;
    //     Ok(())
    // }
    pub async fn read_header<R: tokio::io::AsyncRead + Send + Unpin>(r: &mut R) -> Result<Element, anyhow::Error> {
        let (id, size, header_len) = io::async_::read_element_id_size(r).await
            .context(format!("Element::parse, failed to read_element_id_size()"))?;
        Ok(Element { id, type_: id.type_(), header_len, size })
    }

    pub async fn skip<R: tokio::io::AsyncRead + tokio::io::AsyncSeek + Send + Unpin>(&self, r: &mut R) -> Result<(), anyhow::Error> {
        // if self.type_ == ElementType::Struct { return Ok(()) }
        use tokio::io::AsyncSeekExt;
        r.seek(std::io::SeekFrom::Current(self.size.unwrap() as i64)).await.context(format!("Failed to read_exact(r, {:?})", self.size))?;
        Ok(())
    }

    pub async fn read_body<R: tokio::io::AsyncRead + Send + Unpin>(&self, r: &mut R) -> Result<ElementContent, anyhow::Error> {
        let size = self.size;
        Ok(match self.id.type_() {
            ElementType::UInteger => io::async_::read_uint(r, size.unwrap()).await.map(|val| ElementContent::UInteger(val))?,
            ElementType::Integer => io::async_::read_int(r, size.unwrap()).await.map(|val| ElementContent::Integer(val))?,
            ElementType::Float => io::async_::read_float(r, size.unwrap()).await.map(|val| ElementContent::Float(val))?,
            ElementType::String => io::async_::read_string(r, size.unwrap()).await.map(|val| ElementContent::String(val))?,
            ElementType::Utf8 => io::async_::read_utf8(r, size.unwrap()).await.map(|val| ElementContent::Utf8(val))?,
            ElementType::Binary => io::async_::read_bin(r, size.unwrap()).await.map(|val| ElementContent::Binary(val))?,
            ElementType::Date => io::async_::read_date(r, size.unwrap()).await.map(|val| ElementContent::Date(val))?,

            ElementType::Struct => Self::read_struct(r, size).await.map(|val| ElementContent::Struct(val))?,
        })
    }

    pub async fn read_struct<R: tokio::io::AsyncRead + Send + Unpin>(r: &mut R, mut size: ElementSize) -> Result<Vec<Element>, anyhow::Error> {
        let mut size = size.unwrap();
        let mut elements = Vec::new();
        while size > 0 {
            let e = Element::read_header(r).await.context(format!("Element::parse_struct, failed to read_element_id_size()"))?;
            if e.size.unwrap() > size {
                Err(MatroskaError::InvalidSize(e.id, e.size))?;
            }
            size -= e.size.unwrap();
            elements.push(e);
        }
        Ok(elements)
    }
}
