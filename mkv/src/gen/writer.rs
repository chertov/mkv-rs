
use std::collections::VecDeque;

use super::io::*;
use super::structs::*;
use super::ids::EbmlId;
use tokio::io::AsyncWriteExt;

impl EbmlHeader {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::EbmlHeader as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                EbmlHeaderFields::Version(val) => blocking::write_el_uint(w, EbmlId::Version as u64, &*val.v)?,
                EbmlHeaderFields::ReadVersion(val) => blocking::write_el_uint(w, EbmlId::ReadVersion as u64, &*val.v)?,
                EbmlHeaderFields::DocType(val) => blocking::write_el_utf8(w, EbmlId::DocType as u64, &val.v)?,
                EbmlHeaderFields::DocTypeVersion(val) => blocking::write_el_uint(w, EbmlId::DocTypeVersion as u64, &*val.v)?,
                EbmlHeaderFields::DocTypeReadVersion(val) => blocking::write_el_uint(w, EbmlId::DocTypeReadVersion as u64, &*val.v)?,
                EbmlHeaderFields::DocTypeExtension(val) => val.v.write_blocking(w)?,
                EbmlHeaderFields::DocTypeExtensionName(val) => blocking::write_el_utf8(w, EbmlId::DocTypeExtensionName as u64, &val.v)?,
                EbmlHeaderFields::DocTypeExtensionVersion(val) => blocking::write_el_uint(w, EbmlId::DocTypeExtensionVersion as u64, &*val.v)?,
                EbmlHeaderFields::EbmlMaxIdLength(val) => blocking::write_el_uint(w, EbmlId::EbmlMaxIdLength as u64, &*val.v)?,
                EbmlHeaderFields::EbmlMaxSizeLength(val) => blocking::write_el_uint(w, EbmlId::EbmlMaxSizeLength as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl EbmlHeader {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::EbmlHeader as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                EbmlHeaderFields::Version(val) => async_::write_el_uint(w, EbmlId::Version as u64, &*val.v).await?,
                EbmlHeaderFields::ReadVersion(val) => async_::write_el_uint(w, EbmlId::ReadVersion as u64, &*val.v).await?,
                EbmlHeaderFields::DocType(val) => async_::write_el_utf8(w, EbmlId::DocType as u64, &val.v).await?,
                EbmlHeaderFields::DocTypeVersion(val) => async_::write_el_uint(w, EbmlId::DocTypeVersion as u64, &*val.v).await?,
                EbmlHeaderFields::DocTypeReadVersion(val) => async_::write_el_uint(w, EbmlId::DocTypeReadVersion as u64, &*val.v).await?,
                EbmlHeaderFields::DocTypeExtension(val) => val.v.write(w).await?,
                EbmlHeaderFields::DocTypeExtensionName(val) => async_::write_el_utf8(w, EbmlId::DocTypeExtensionName as u64, &val.v).await?,
                EbmlHeaderFields::DocTypeExtensionVersion(val) => async_::write_el_uint(w, EbmlId::DocTypeExtensionVersion as u64, &*val.v).await?,
                EbmlHeaderFields::EbmlMaxIdLength(val) => async_::write_el_uint(w, EbmlId::EbmlMaxIdLength as u64, &*val.v).await?,
                EbmlHeaderFields::EbmlMaxSizeLength(val) => async_::write_el_uint(w, EbmlId::EbmlMaxSizeLength as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl DocTypeExtension {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::DocTypeExtension as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        Ok(size)
    }
}
impl DocTypeExtension {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::DocTypeExtension as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        Ok(size)
    }
}

impl Segment {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::Segment as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                SegmentFields::Void(val) => blocking::write_el_bin(w, EbmlId::Void as u64, &val.v)?,
                SegmentFields::Crc32(val) => blocking::write_el_bin(w, EbmlId::Crc32 as u64, &val.v)?,
                SegmentFields::SeekHead(val) => val.v.write_blocking(w)?,
                SegmentFields::Info(val) => val.v.write_blocking(w)?,
                SegmentFields::Cluster(val) => val.v.write_blocking(w)?,
                SegmentFields::Tracks(val) => val.v.write_blocking(w)?,
                SegmentFields::Cues(val) => val.v.write_blocking(w)?,
                SegmentFields::Attachments(val) => val.v.write_blocking(w)?,
                SegmentFields::Chapters(val) => val.v.write_blocking(w)?,
                SegmentFields::Tags(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl Segment {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::Segment as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                SegmentFields::Void(val) => async_::write_el_bin(w, EbmlId::Void as u64, &val.v).await?,
                SegmentFields::Crc32(val) => async_::write_el_bin(w, EbmlId::Crc32 as u64, &val.v).await?,
                SegmentFields::SeekHead(val) => val.v.write(w).await?,
                SegmentFields::Info(val) => val.v.write(w).await?,
                SegmentFields::Cluster(val) => val.v.write(w).await?,
                SegmentFields::Tracks(val) => val.v.write(w).await?,
                SegmentFields::Cues(val) => val.v.write(w).await?,
                SegmentFields::Attachments(val) => val.v.write(w).await?,
                SegmentFields::Chapters(val) => val.v.write(w).await?,
                SegmentFields::Tags(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl SeekHead {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::SeekHead as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                SeekHeadFields::Seek(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl SeekHead {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::SeekHead as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                SeekHeadFields::Seek(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl Seek {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::Seek as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                SeekFields::SeekId(val) => blocking::write_el_bin(w, EbmlId::SeekId as u64, &val.v)?,
                SeekFields::SeekPosition(val) => blocking::write_el_uint(w, EbmlId::SeekPosition as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl Seek {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::Seek as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                SeekFields::SeekId(val) => async_::write_el_bin(w, EbmlId::SeekId as u64, &val.v).await?,
                SeekFields::SeekPosition(val) => async_::write_el_uint(w, EbmlId::SeekPosition as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl Info {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::Info as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                InfoFields::SegmentUuid(val) => blocking::write_el_bin(w, EbmlId::SegmentUuid as u64, &val.v)?,
                InfoFields::SegmentFilename(val) => blocking::write_el_utf8(w, EbmlId::SegmentFilename as u64, &val.v)?,
                InfoFields::PrevUuid(val) => blocking::write_el_bin(w, EbmlId::PrevUuid as u64, &val.v)?,
                InfoFields::PrevFilename(val) => blocking::write_el_utf8(w, EbmlId::PrevFilename as u64, &val.v)?,
                InfoFields::NextUuid(val) => blocking::write_el_bin(w, EbmlId::NextUuid as u64, &val.v)?,
                InfoFields::NextFilename(val) => blocking::write_el_utf8(w, EbmlId::NextFilename as u64, &val.v)?,
                InfoFields::SegmentFamily(val) => blocking::write_el_bin(w, EbmlId::SegmentFamily as u64, &val.v)?,
                InfoFields::ChapterTranslate(val) => val.v.write_blocking(w)?,
                InfoFields::TimestampScale(val) => blocking::write_el_uint(w, EbmlId::TimestampScale as u64, &*val.v)?,
                InfoFields::Duration(val) => blocking::write_el_float64(w, EbmlId::Duration as u64, &*val.v)?,
                InfoFields::DateUtc(val) => blocking::write_el_date(w, EbmlId::DateUtc as u64, &val.v)?,
                InfoFields::Title(val) => blocking::write_el_utf8(w, EbmlId::Title as u64, &val.v)?,
                InfoFields::MuxingApp(val) => blocking::write_el_utf8(w, EbmlId::MuxingApp as u64, &val.v)?,
                InfoFields::WritingApp(val) => blocking::write_el_utf8(w, EbmlId::WritingApp as u64, &val.v)?,
            }
        }
        Ok(size)
    }
}
impl Info {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::Info as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                InfoFields::SegmentUuid(val) => async_::write_el_bin(w, EbmlId::SegmentUuid as u64, &val.v).await?,
                InfoFields::SegmentFilename(val) => async_::write_el_utf8(w, EbmlId::SegmentFilename as u64, &val.v).await?,
                InfoFields::PrevUuid(val) => async_::write_el_bin(w, EbmlId::PrevUuid as u64, &val.v).await?,
                InfoFields::PrevFilename(val) => async_::write_el_utf8(w, EbmlId::PrevFilename as u64, &val.v).await?,
                InfoFields::NextUuid(val) => async_::write_el_bin(w, EbmlId::NextUuid as u64, &val.v).await?,
                InfoFields::NextFilename(val) => async_::write_el_utf8(w, EbmlId::NextFilename as u64, &val.v).await?,
                InfoFields::SegmentFamily(val) => async_::write_el_bin(w, EbmlId::SegmentFamily as u64, &val.v).await?,
                InfoFields::ChapterTranslate(val) => val.v.write(w).await?,
                InfoFields::TimestampScale(val) => async_::write_el_uint(w, EbmlId::TimestampScale as u64, &*val.v).await?,
                InfoFields::Duration(val) => async_::write_el_float64(w, EbmlId::Duration as u64, &*val.v).await?,
                InfoFields::DateUtc(val) => async_::write_el_date(w, EbmlId::DateUtc as u64, &val.v).await?,
                InfoFields::Title(val) => async_::write_el_utf8(w, EbmlId::Title as u64, &val.v).await?,
                InfoFields::MuxingApp(val) => async_::write_el_utf8(w, EbmlId::MuxingApp as u64, &val.v).await?,
                InfoFields::WritingApp(val) => async_::write_el_utf8(w, EbmlId::WritingApp as u64, &val.v).await?,
            }
        }
        Ok(size)
    }
}

impl ChapterTranslate {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::ChapterTranslate as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ChapterTranslateFields::ChapterTranslateId(val) => blocking::write_el_bin(w, EbmlId::ChapterTranslateId as u64, &val.v)?,
                ChapterTranslateFields::ChapterTranslateCodec(val) => blocking::write_el_uint(w, EbmlId::ChapterTranslateCodec as u64, &*val.v)?,
                ChapterTranslateFields::ChapterTranslateEditionUid(val) => blocking::write_el_uint(w, EbmlId::ChapterTranslateEditionUid as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl ChapterTranslate {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::ChapterTranslate as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ChapterTranslateFields::ChapterTranslateId(val) => async_::write_el_bin(w, EbmlId::ChapterTranslateId as u64, &val.v).await?,
                ChapterTranslateFields::ChapterTranslateCodec(val) => async_::write_el_uint(w, EbmlId::ChapterTranslateCodec as u64, &*val.v).await?,
                ChapterTranslateFields::ChapterTranslateEditionUid(val) => async_::write_el_uint(w, EbmlId::ChapterTranslateEditionUid as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl Cluster {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::Cluster as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ClusterFields::Timestamp(val) => blocking::write_el_uint(w, EbmlId::Timestamp as u64, &*val.v)?,
                ClusterFields::SilentTracks(val) => val.v.write_blocking(w)?,
                ClusterFields::Position(val) => blocking::write_el_uint(w, EbmlId::Position as u64, &*val.v)?,
                ClusterFields::PrevSize(val) => blocking::write_el_uint(w, EbmlId::PrevSize as u64, &*val.v)?,
                ClusterFields::SimpleBlock(val) => blocking::write_el_bin(w, EbmlId::SimpleBlock as u64, &val.v)?,
                ClusterFields::BlockGroup(val) => val.v.write_blocking(w)?,
                ClusterFields::EncryptedBlock(val) => blocking::write_el_bin(w, EbmlId::EncryptedBlock as u64, &val.v)?,
            }
        }
        Ok(size)
    }
}
impl Cluster {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::Cluster as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ClusterFields::Timestamp(val) => async_::write_el_uint(w, EbmlId::Timestamp as u64, &*val.v).await?,
                ClusterFields::SilentTracks(val) => val.v.write(w).await?,
                ClusterFields::Position(val) => async_::write_el_uint(w, EbmlId::Position as u64, &*val.v).await?,
                ClusterFields::PrevSize(val) => async_::write_el_uint(w, EbmlId::PrevSize as u64, &*val.v).await?,
                ClusterFields::SimpleBlock(val) => async_::write_el_bin(w, EbmlId::SimpleBlock as u64, &val.v).await?,
                ClusterFields::BlockGroup(val) => val.v.write(w).await?,
                ClusterFields::EncryptedBlock(val) => async_::write_el_bin(w, EbmlId::EncryptedBlock as u64, &val.v).await?,
            }
        }
        Ok(size)
    }
}

impl SilentTracks {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::SilentTracks as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                SilentTracksFields::SilentTrackNumber(val) => blocking::write_el_uint(w, EbmlId::SilentTrackNumber as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl SilentTracks {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::SilentTracks as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                SilentTracksFields::SilentTrackNumber(val) => async_::write_el_uint(w, EbmlId::SilentTrackNumber as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl BlockGroup {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::BlockGroup as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                BlockGroupFields::Block(val) => blocking::write_el_bin(w, EbmlId::Block as u64, &val.v)?,
                BlockGroupFields::BlockVirtual(val) => blocking::write_el_bin(w, EbmlId::BlockVirtual as u64, &val.v)?,
                BlockGroupFields::BlockAdditions(val) => val.v.write_blocking(w)?,
                BlockGroupFields::BlockDuration(val) => blocking::write_el_uint(w, EbmlId::BlockDuration as u64, &*val.v)?,
                BlockGroupFields::ReferencePriority(val) => blocking::write_el_uint(w, EbmlId::ReferencePriority as u64, &*val.v)?,
                BlockGroupFields::ReferenceBlock(val) => blocking::write_el_int(w, EbmlId::ReferenceBlock as u64, &*val.v)?,
                BlockGroupFields::ReferenceVirtual(val) => blocking::write_el_int(w, EbmlId::ReferenceVirtual as u64, &*val.v)?,
                BlockGroupFields::CodecState(val) => blocking::write_el_bin(w, EbmlId::CodecState as u64, &val.v)?,
                BlockGroupFields::DiscardPadding(val) => blocking::write_el_int(w, EbmlId::DiscardPadding as u64, &*val.v)?,
                BlockGroupFields::Slices(val) => val.v.write_blocking(w)?,
                BlockGroupFields::ReferenceFrame(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl BlockGroup {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::BlockGroup as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                BlockGroupFields::Block(val) => async_::write_el_bin(w, EbmlId::Block as u64, &val.v).await?,
                BlockGroupFields::BlockVirtual(val) => async_::write_el_bin(w, EbmlId::BlockVirtual as u64, &val.v).await?,
                BlockGroupFields::BlockAdditions(val) => val.v.write(w).await?,
                BlockGroupFields::BlockDuration(val) => async_::write_el_uint(w, EbmlId::BlockDuration as u64, &*val.v).await?,
                BlockGroupFields::ReferencePriority(val) => async_::write_el_uint(w, EbmlId::ReferencePriority as u64, &*val.v).await?,
                BlockGroupFields::ReferenceBlock(val) => async_::write_el_int(w, EbmlId::ReferenceBlock as u64, &*val.v).await?,
                BlockGroupFields::ReferenceVirtual(val) => async_::write_el_int(w, EbmlId::ReferenceVirtual as u64, &*val.v).await?,
                BlockGroupFields::CodecState(val) => async_::write_el_bin(w, EbmlId::CodecState as u64, &val.v).await?,
                BlockGroupFields::DiscardPadding(val) => async_::write_el_int(w, EbmlId::DiscardPadding as u64, &*val.v).await?,
                BlockGroupFields::Slices(val) => val.v.write(w).await?,
                BlockGroupFields::ReferenceFrame(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl BlockAdditions {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::BlockAdditions as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                BlockAdditionsFields::BlockMore(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl BlockAdditions {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::BlockAdditions as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                BlockAdditionsFields::BlockMore(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl BlockMore {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::BlockMore as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                BlockMoreFields::BlockAdditional(val) => blocking::write_el_bin(w, EbmlId::BlockAdditional as u64, &val.v)?,
                BlockMoreFields::BlockAddId(val) => blocking::write_el_uint(w, EbmlId::BlockAddId as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl BlockMore {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::BlockMore as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                BlockMoreFields::BlockAdditional(val) => async_::write_el_bin(w, EbmlId::BlockAdditional as u64, &val.v).await?,
                BlockMoreFields::BlockAddId(val) => async_::write_el_uint(w, EbmlId::BlockAddId as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl Slices {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::Slices as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                SlicesFields::TimeSlice(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl Slices {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::Slices as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                SlicesFields::TimeSlice(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl TimeSlice {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::TimeSlice as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TimeSliceFields::LaceNumber(val) => blocking::write_el_uint(w, EbmlId::LaceNumber as u64, &*val.v)?,
                TimeSliceFields::FrameNumber(val) => blocking::write_el_uint(w, EbmlId::FrameNumber as u64, &*val.v)?,
                TimeSliceFields::BlockAdditionId(val) => blocking::write_el_uint(w, EbmlId::BlockAdditionId as u64, &*val.v)?,
                TimeSliceFields::Delay(val) => blocking::write_el_uint(w, EbmlId::Delay as u64, &*val.v)?,
                TimeSliceFields::SliceDuration(val) => blocking::write_el_uint(w, EbmlId::SliceDuration as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl TimeSlice {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::TimeSlice as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TimeSliceFields::LaceNumber(val) => async_::write_el_uint(w, EbmlId::LaceNumber as u64, &*val.v).await?,
                TimeSliceFields::FrameNumber(val) => async_::write_el_uint(w, EbmlId::FrameNumber as u64, &*val.v).await?,
                TimeSliceFields::BlockAdditionId(val) => async_::write_el_uint(w, EbmlId::BlockAdditionId as u64, &*val.v).await?,
                TimeSliceFields::Delay(val) => async_::write_el_uint(w, EbmlId::Delay as u64, &*val.v).await?,
                TimeSliceFields::SliceDuration(val) => async_::write_el_uint(w, EbmlId::SliceDuration as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl ReferenceFrame {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::ReferenceFrame as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ReferenceFrameFields::ReferenceOffset(val) => blocking::write_el_uint(w, EbmlId::ReferenceOffset as u64, &*val.v)?,
                ReferenceFrameFields::ReferenceTimestamp(val) => blocking::write_el_uint(w, EbmlId::ReferenceTimestamp as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl ReferenceFrame {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::ReferenceFrame as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ReferenceFrameFields::ReferenceOffset(val) => async_::write_el_uint(w, EbmlId::ReferenceOffset as u64, &*val.v).await?,
                ReferenceFrameFields::ReferenceTimestamp(val) => async_::write_el_uint(w, EbmlId::ReferenceTimestamp as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl Tracks {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::Tracks as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TracksFields::TrackEntry(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl Tracks {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::Tracks as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TracksFields::TrackEntry(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl TrackEntry {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::TrackEntry as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TrackEntryFields::TrackNumber(val) => blocking::write_el_uint(w, EbmlId::TrackNumber as u64, &*val.v)?,
                TrackEntryFields::TrackUid(val) => blocking::write_el_uint(w, EbmlId::TrackUid as u64, &*val.v)?,
                TrackEntryFields::TrackType(val) => blocking::write_el_uint(w, EbmlId::TrackType as u64, &*val.v)?,
                TrackEntryFields::FlagEnabled(val) => blocking::write_el_uint(w, EbmlId::FlagEnabled as u64, &*val.v)?,
                TrackEntryFields::FlagDefault(val) => blocking::write_el_uint(w, EbmlId::FlagDefault as u64, &*val.v)?,
                TrackEntryFields::FlagForced(val) => blocking::write_el_uint(w, EbmlId::FlagForced as u64, &*val.v)?,
                TrackEntryFields::FlagHearingImpaired(val) => blocking::write_el_uint(w, EbmlId::FlagHearingImpaired as u64, &*val.v)?,
                TrackEntryFields::FlagVisualImpaired(val) => blocking::write_el_uint(w, EbmlId::FlagVisualImpaired as u64, &*val.v)?,
                TrackEntryFields::FlagTextDescriptions(val) => blocking::write_el_uint(w, EbmlId::FlagTextDescriptions as u64, &*val.v)?,
                TrackEntryFields::FlagOriginal(val) => blocking::write_el_uint(w, EbmlId::FlagOriginal as u64, &*val.v)?,
                TrackEntryFields::FlagCommentary(val) => blocking::write_el_uint(w, EbmlId::FlagCommentary as u64, &*val.v)?,
                TrackEntryFields::FlagLacing(val) => blocking::write_el_uint(w, EbmlId::FlagLacing as u64, &*val.v)?,
                TrackEntryFields::MinCache(val) => blocking::write_el_uint(w, EbmlId::MinCache as u64, &*val.v)?,
                TrackEntryFields::MaxCache(val) => blocking::write_el_uint(w, EbmlId::MaxCache as u64, &*val.v)?,
                TrackEntryFields::DefaultDuration(val) => blocking::write_el_uint(w, EbmlId::DefaultDuration as u64, &*val.v)?,
                TrackEntryFields::DefaultDecodedFieldDuration(val) => blocking::write_el_uint(w, EbmlId::DefaultDecodedFieldDuration as u64, &*val.v)?,
                TrackEntryFields::TrackTimestampScale(val) => blocking::write_el_float64(w, EbmlId::TrackTimestampScale as u64, &*val.v)?,
                TrackEntryFields::TrackOffset(val) => blocking::write_el_int(w, EbmlId::TrackOffset as u64, &*val.v)?,
                TrackEntryFields::MaxBlockAdditionId(val) => blocking::write_el_uint(w, EbmlId::MaxBlockAdditionId as u64, &*val.v)?,
                TrackEntryFields::BlockAdditionMapping(val) => val.v.write_blocking(w)?,
                TrackEntryFields::Name(val) => blocking::write_el_utf8(w, EbmlId::Name as u64, &val.v)?,
                TrackEntryFields::Language(val) => blocking::write_el_string(w, EbmlId::Language as u64, &val.v)?,
                TrackEntryFields::LanguageBcp47(val) => blocking::write_el_string(w, EbmlId::LanguageBcp47 as u64, &val.v)?,
                TrackEntryFields::CodecId(val) => blocking::write_el_string(w, EbmlId::CodecId as u64, &val.v)?,
                TrackEntryFields::CodecPrivate(val) => blocking::write_el_bin(w, EbmlId::CodecPrivate as u64, &val.v)?,
                TrackEntryFields::CodecName(val) => blocking::write_el_utf8(w, EbmlId::CodecName as u64, &val.v)?,
                TrackEntryFields::AttachmentLink(val) => blocking::write_el_uint(w, EbmlId::AttachmentLink as u64, &*val.v)?,
                TrackEntryFields::CodecSettings(val) => blocking::write_el_utf8(w, EbmlId::CodecSettings as u64, &val.v)?,
                TrackEntryFields::CodecInfoUrl(val) => blocking::write_el_string(w, EbmlId::CodecInfoUrl as u64, &val.v)?,
                TrackEntryFields::CodecDownloadUrl(val) => blocking::write_el_string(w, EbmlId::CodecDownloadUrl as u64, &val.v)?,
                TrackEntryFields::CodecDecodeAll(val) => blocking::write_el_uint(w, EbmlId::CodecDecodeAll as u64, &*val.v)?,
                TrackEntryFields::TrackOverlay(val) => blocking::write_el_uint(w, EbmlId::TrackOverlay as u64, &*val.v)?,
                TrackEntryFields::CodecDelay(val) => blocking::write_el_uint(w, EbmlId::CodecDelay as u64, &*val.v)?,
                TrackEntryFields::SeekPreRoll(val) => blocking::write_el_uint(w, EbmlId::SeekPreRoll as u64, &*val.v)?,
                TrackEntryFields::TrackTranslate(val) => val.v.write_blocking(w)?,
                TrackEntryFields::Video(val) => val.v.write_blocking(w)?,
                TrackEntryFields::Audio(val) => val.v.write_blocking(w)?,
                TrackEntryFields::TrackOperation(val) => val.v.write_blocking(w)?,
                TrackEntryFields::TrickTrackUid(val) => blocking::write_el_uint(w, EbmlId::TrickTrackUid as u64, &*val.v)?,
                TrackEntryFields::TrickTrackSegmentUid(val) => blocking::write_el_bin(w, EbmlId::TrickTrackSegmentUid as u64, &val.v)?,
                TrackEntryFields::TrickTrackFlag(val) => blocking::write_el_uint(w, EbmlId::TrickTrackFlag as u64, &*val.v)?,
                TrackEntryFields::TrickStructTrackUid(val) => blocking::write_el_uint(w, EbmlId::TrickStructTrackUid as u64, &*val.v)?,
                TrackEntryFields::TrickStructTrackSegmentUid(val) => blocking::write_el_bin(w, EbmlId::TrickStructTrackSegmentUid as u64, &val.v)?,
                TrackEntryFields::ContentEncodings(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl TrackEntry {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::TrackEntry as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TrackEntryFields::TrackNumber(val) => async_::write_el_uint(w, EbmlId::TrackNumber as u64, &*val.v).await?,
                TrackEntryFields::TrackUid(val) => async_::write_el_uint(w, EbmlId::TrackUid as u64, &*val.v).await?,
                TrackEntryFields::TrackType(val) => async_::write_el_uint(w, EbmlId::TrackType as u64, &*val.v).await?,
                TrackEntryFields::FlagEnabled(val) => async_::write_el_uint(w, EbmlId::FlagEnabled as u64, &*val.v).await?,
                TrackEntryFields::FlagDefault(val) => async_::write_el_uint(w, EbmlId::FlagDefault as u64, &*val.v).await?,
                TrackEntryFields::FlagForced(val) => async_::write_el_uint(w, EbmlId::FlagForced as u64, &*val.v).await?,
                TrackEntryFields::FlagHearingImpaired(val) => async_::write_el_uint(w, EbmlId::FlagHearingImpaired as u64, &*val.v).await?,
                TrackEntryFields::FlagVisualImpaired(val) => async_::write_el_uint(w, EbmlId::FlagVisualImpaired as u64, &*val.v).await?,
                TrackEntryFields::FlagTextDescriptions(val) => async_::write_el_uint(w, EbmlId::FlagTextDescriptions as u64, &*val.v).await?,
                TrackEntryFields::FlagOriginal(val) => async_::write_el_uint(w, EbmlId::FlagOriginal as u64, &*val.v).await?,
                TrackEntryFields::FlagCommentary(val) => async_::write_el_uint(w, EbmlId::FlagCommentary as u64, &*val.v).await?,
                TrackEntryFields::FlagLacing(val) => async_::write_el_uint(w, EbmlId::FlagLacing as u64, &*val.v).await?,
                TrackEntryFields::MinCache(val) => async_::write_el_uint(w, EbmlId::MinCache as u64, &*val.v).await?,
                TrackEntryFields::MaxCache(val) => async_::write_el_uint(w, EbmlId::MaxCache as u64, &*val.v).await?,
                TrackEntryFields::DefaultDuration(val) => async_::write_el_uint(w, EbmlId::DefaultDuration as u64, &*val.v).await?,
                TrackEntryFields::DefaultDecodedFieldDuration(val) => async_::write_el_uint(w, EbmlId::DefaultDecodedFieldDuration as u64, &*val.v).await?,
                TrackEntryFields::TrackTimestampScale(val) => async_::write_el_float64(w, EbmlId::TrackTimestampScale as u64, &*val.v).await?,
                TrackEntryFields::TrackOffset(val) => async_::write_el_int(w, EbmlId::TrackOffset as u64, &*val.v).await?,
                TrackEntryFields::MaxBlockAdditionId(val) => async_::write_el_uint(w, EbmlId::MaxBlockAdditionId as u64, &*val.v).await?,
                TrackEntryFields::BlockAdditionMapping(val) => val.v.write(w).await?,
                TrackEntryFields::Name(val) => async_::write_el_utf8(w, EbmlId::Name as u64, &val.v).await?,
                TrackEntryFields::Language(val) => async_::write_el_string(w, EbmlId::Language as u64, &val.v).await?,
                TrackEntryFields::LanguageBcp47(val) => async_::write_el_string(w, EbmlId::LanguageBcp47 as u64, &val.v).await?,
                TrackEntryFields::CodecId(val) => async_::write_el_string(w, EbmlId::CodecId as u64, &val.v).await?,
                TrackEntryFields::CodecPrivate(val) => async_::write_el_bin(w, EbmlId::CodecPrivate as u64, &val.v).await?,
                TrackEntryFields::CodecName(val) => async_::write_el_utf8(w, EbmlId::CodecName as u64, &val.v).await?,
                TrackEntryFields::AttachmentLink(val) => async_::write_el_uint(w, EbmlId::AttachmentLink as u64, &*val.v).await?,
                TrackEntryFields::CodecSettings(val) => async_::write_el_utf8(w, EbmlId::CodecSettings as u64, &val.v).await?,
                TrackEntryFields::CodecInfoUrl(val) => async_::write_el_string(w, EbmlId::CodecInfoUrl as u64, &val.v).await?,
                TrackEntryFields::CodecDownloadUrl(val) => async_::write_el_string(w, EbmlId::CodecDownloadUrl as u64, &val.v).await?,
                TrackEntryFields::CodecDecodeAll(val) => async_::write_el_uint(w, EbmlId::CodecDecodeAll as u64, &*val.v).await?,
                TrackEntryFields::TrackOverlay(val) => async_::write_el_uint(w, EbmlId::TrackOverlay as u64, &*val.v).await?,
                TrackEntryFields::CodecDelay(val) => async_::write_el_uint(w, EbmlId::CodecDelay as u64, &*val.v).await?,
                TrackEntryFields::SeekPreRoll(val) => async_::write_el_uint(w, EbmlId::SeekPreRoll as u64, &*val.v).await?,
                TrackEntryFields::TrackTranslate(val) => val.v.write(w).await?,
                TrackEntryFields::Video(val) => val.v.write(w).await?,
                TrackEntryFields::Audio(val) => val.v.write(w).await?,
                TrackEntryFields::TrackOperation(val) => val.v.write(w).await?,
                TrackEntryFields::TrickTrackUid(val) => async_::write_el_uint(w, EbmlId::TrickTrackUid as u64, &*val.v).await?,
                TrackEntryFields::TrickTrackSegmentUid(val) => async_::write_el_bin(w, EbmlId::TrickTrackSegmentUid as u64, &val.v).await?,
                TrackEntryFields::TrickTrackFlag(val) => async_::write_el_uint(w, EbmlId::TrickTrackFlag as u64, &*val.v).await?,
                TrackEntryFields::TrickStructTrackUid(val) => async_::write_el_uint(w, EbmlId::TrickStructTrackUid as u64, &*val.v).await?,
                TrackEntryFields::TrickStructTrackSegmentUid(val) => async_::write_el_bin(w, EbmlId::TrickStructTrackSegmentUid as u64, &val.v).await?,
                TrackEntryFields::ContentEncodings(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl BlockAdditionMapping {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::BlockAdditionMapping as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                BlockAdditionMappingFields::BlockAddIdValue(val) => blocking::write_el_uint(w, EbmlId::BlockAddIdValue as u64, &*val.v)?,
                BlockAdditionMappingFields::BlockAddIdName(val) => blocking::write_el_string(w, EbmlId::BlockAddIdName as u64, &val.v)?,
                BlockAdditionMappingFields::BlockAddIdType(val) => blocking::write_el_uint(w, EbmlId::BlockAddIdType as u64, &*val.v)?,
                BlockAdditionMappingFields::BlockAddIdExtraData(val) => blocking::write_el_bin(w, EbmlId::BlockAddIdExtraData as u64, &val.v)?,
            }
        }
        Ok(size)
    }
}
impl BlockAdditionMapping {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::BlockAdditionMapping as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                BlockAdditionMappingFields::BlockAddIdValue(val) => async_::write_el_uint(w, EbmlId::BlockAddIdValue as u64, &*val.v).await?,
                BlockAdditionMappingFields::BlockAddIdName(val) => async_::write_el_string(w, EbmlId::BlockAddIdName as u64, &val.v).await?,
                BlockAdditionMappingFields::BlockAddIdType(val) => async_::write_el_uint(w, EbmlId::BlockAddIdType as u64, &*val.v).await?,
                BlockAdditionMappingFields::BlockAddIdExtraData(val) => async_::write_el_bin(w, EbmlId::BlockAddIdExtraData as u64, &val.v).await?,
            }
        }
        Ok(size)
    }
}

impl TrackTranslate {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::TrackTranslate as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TrackTranslateFields::TrackTranslateTrackId(val) => blocking::write_el_bin(w, EbmlId::TrackTranslateTrackId as u64, &val.v)?,
                TrackTranslateFields::TrackTranslateCodec(val) => blocking::write_el_uint(w, EbmlId::TrackTranslateCodec as u64, &*val.v)?,
                TrackTranslateFields::TrackTranslateEditionUid(val) => blocking::write_el_uint(w, EbmlId::TrackTranslateEditionUid as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl TrackTranslate {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::TrackTranslate as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TrackTranslateFields::TrackTranslateTrackId(val) => async_::write_el_bin(w, EbmlId::TrackTranslateTrackId as u64, &val.v).await?,
                TrackTranslateFields::TrackTranslateCodec(val) => async_::write_el_uint(w, EbmlId::TrackTranslateCodec as u64, &*val.v).await?,
                TrackTranslateFields::TrackTranslateEditionUid(val) => async_::write_el_uint(w, EbmlId::TrackTranslateEditionUid as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl Video {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::Video as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                VideoFields::FlagInterlaced(val) => blocking::write_el_uint(w, EbmlId::FlagInterlaced as u64, &*val.v)?,
                VideoFields::FieldOrder(val) => blocking::write_el_uint(w, EbmlId::FieldOrder as u64, &*val.v)?,
                VideoFields::StereoMode(val) => blocking::write_el_uint(w, EbmlId::StereoMode as u64, &*val.v)?,
                VideoFields::AlphaMode(val) => blocking::write_el_uint(w, EbmlId::AlphaMode as u64, &*val.v)?,
                VideoFields::OldStereoMode(val) => blocking::write_el_uint(w, EbmlId::OldStereoMode as u64, &*val.v)?,
                VideoFields::PixelWidth(val) => blocking::write_el_uint(w, EbmlId::PixelWidth as u64, &*val.v)?,
                VideoFields::PixelHeight(val) => blocking::write_el_uint(w, EbmlId::PixelHeight as u64, &*val.v)?,
                VideoFields::PixelCropBottom(val) => blocking::write_el_uint(w, EbmlId::PixelCropBottom as u64, &*val.v)?,
                VideoFields::PixelCropTop(val) => blocking::write_el_uint(w, EbmlId::PixelCropTop as u64, &*val.v)?,
                VideoFields::PixelCropLeft(val) => blocking::write_el_uint(w, EbmlId::PixelCropLeft as u64, &*val.v)?,
                VideoFields::PixelCropRight(val) => blocking::write_el_uint(w, EbmlId::PixelCropRight as u64, &*val.v)?,
                VideoFields::DisplayWidth(val) => blocking::write_el_uint(w, EbmlId::DisplayWidth as u64, &*val.v)?,
                VideoFields::DisplayHeight(val) => blocking::write_el_uint(w, EbmlId::DisplayHeight as u64, &*val.v)?,
                VideoFields::DisplayUnit(val) => blocking::write_el_uint(w, EbmlId::DisplayUnit as u64, &*val.v)?,
                VideoFields::AspectRatioType(val) => blocking::write_el_uint(w, EbmlId::AspectRatioType as u64, &*val.v)?,
                VideoFields::UncompressedFourCc(val) => blocking::write_el_bin(w, EbmlId::UncompressedFourCc as u64, &val.v)?,
                VideoFields::GammaValue(val) => blocking::write_el_float64(w, EbmlId::GammaValue as u64, &*val.v)?,
                VideoFields::FrameRate(val) => blocking::write_el_float64(w, EbmlId::FrameRate as u64, &*val.v)?,
                VideoFields::Colour(val) => val.v.write_blocking(w)?,
                VideoFields::Projection(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl Video {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::Video as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                VideoFields::FlagInterlaced(val) => async_::write_el_uint(w, EbmlId::FlagInterlaced as u64, &*val.v).await?,
                VideoFields::FieldOrder(val) => async_::write_el_uint(w, EbmlId::FieldOrder as u64, &*val.v).await?,
                VideoFields::StereoMode(val) => async_::write_el_uint(w, EbmlId::StereoMode as u64, &*val.v).await?,
                VideoFields::AlphaMode(val) => async_::write_el_uint(w, EbmlId::AlphaMode as u64, &*val.v).await?,
                VideoFields::OldStereoMode(val) => async_::write_el_uint(w, EbmlId::OldStereoMode as u64, &*val.v).await?,
                VideoFields::PixelWidth(val) => async_::write_el_uint(w, EbmlId::PixelWidth as u64, &*val.v).await?,
                VideoFields::PixelHeight(val) => async_::write_el_uint(w, EbmlId::PixelHeight as u64, &*val.v).await?,
                VideoFields::PixelCropBottom(val) => async_::write_el_uint(w, EbmlId::PixelCropBottom as u64, &*val.v).await?,
                VideoFields::PixelCropTop(val) => async_::write_el_uint(w, EbmlId::PixelCropTop as u64, &*val.v).await?,
                VideoFields::PixelCropLeft(val) => async_::write_el_uint(w, EbmlId::PixelCropLeft as u64, &*val.v).await?,
                VideoFields::PixelCropRight(val) => async_::write_el_uint(w, EbmlId::PixelCropRight as u64, &*val.v).await?,
                VideoFields::DisplayWidth(val) => async_::write_el_uint(w, EbmlId::DisplayWidth as u64, &*val.v).await?,
                VideoFields::DisplayHeight(val) => async_::write_el_uint(w, EbmlId::DisplayHeight as u64, &*val.v).await?,
                VideoFields::DisplayUnit(val) => async_::write_el_uint(w, EbmlId::DisplayUnit as u64, &*val.v).await?,
                VideoFields::AspectRatioType(val) => async_::write_el_uint(w, EbmlId::AspectRatioType as u64, &*val.v).await?,
                VideoFields::UncompressedFourCc(val) => async_::write_el_bin(w, EbmlId::UncompressedFourCc as u64, &val.v).await?,
                VideoFields::GammaValue(val) => async_::write_el_float64(w, EbmlId::GammaValue as u64, &*val.v).await?,
                VideoFields::FrameRate(val) => async_::write_el_float64(w, EbmlId::FrameRate as u64, &*val.v).await?,
                VideoFields::Colour(val) => val.v.write(w).await?,
                VideoFields::Projection(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl Colour {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::Colour as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ColourFields::MatrixCoefficients(val) => blocking::write_el_uint(w, EbmlId::MatrixCoefficients as u64, &*val.v)?,
                ColourFields::BitsPerChannel(val) => blocking::write_el_uint(w, EbmlId::BitsPerChannel as u64, &*val.v)?,
                ColourFields::ChromaSubsamplingHorz(val) => blocking::write_el_uint(w, EbmlId::ChromaSubsamplingHorz as u64, &*val.v)?,
                ColourFields::ChromaSubsamplingVert(val) => blocking::write_el_uint(w, EbmlId::ChromaSubsamplingVert as u64, &*val.v)?,
                ColourFields::CbSubsamplingHorz(val) => blocking::write_el_uint(w, EbmlId::CbSubsamplingHorz as u64, &*val.v)?,
                ColourFields::CbSubsamplingVert(val) => blocking::write_el_uint(w, EbmlId::CbSubsamplingVert as u64, &*val.v)?,
                ColourFields::ChromaSitingHorz(val) => blocking::write_el_uint(w, EbmlId::ChromaSitingHorz as u64, &*val.v)?,
                ColourFields::ChromaSitingVert(val) => blocking::write_el_uint(w, EbmlId::ChromaSitingVert as u64, &*val.v)?,
                ColourFields::Range(val) => blocking::write_el_uint(w, EbmlId::Range as u64, &*val.v)?,
                ColourFields::TransferCharacteristics(val) => blocking::write_el_uint(w, EbmlId::TransferCharacteristics as u64, &*val.v)?,
                ColourFields::Primaries(val) => blocking::write_el_uint(w, EbmlId::Primaries as u64, &*val.v)?,
                ColourFields::MaxCll(val) => blocking::write_el_uint(w, EbmlId::MaxCll as u64, &*val.v)?,
                ColourFields::MaxFall(val) => blocking::write_el_uint(w, EbmlId::MaxFall as u64, &*val.v)?,
                ColourFields::StructingMetadata(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl Colour {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::Colour as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ColourFields::MatrixCoefficients(val) => async_::write_el_uint(w, EbmlId::MatrixCoefficients as u64, &*val.v).await?,
                ColourFields::BitsPerChannel(val) => async_::write_el_uint(w, EbmlId::BitsPerChannel as u64, &*val.v).await?,
                ColourFields::ChromaSubsamplingHorz(val) => async_::write_el_uint(w, EbmlId::ChromaSubsamplingHorz as u64, &*val.v).await?,
                ColourFields::ChromaSubsamplingVert(val) => async_::write_el_uint(w, EbmlId::ChromaSubsamplingVert as u64, &*val.v).await?,
                ColourFields::CbSubsamplingHorz(val) => async_::write_el_uint(w, EbmlId::CbSubsamplingHorz as u64, &*val.v).await?,
                ColourFields::CbSubsamplingVert(val) => async_::write_el_uint(w, EbmlId::CbSubsamplingVert as u64, &*val.v).await?,
                ColourFields::ChromaSitingHorz(val) => async_::write_el_uint(w, EbmlId::ChromaSitingHorz as u64, &*val.v).await?,
                ColourFields::ChromaSitingVert(val) => async_::write_el_uint(w, EbmlId::ChromaSitingVert as u64, &*val.v).await?,
                ColourFields::Range(val) => async_::write_el_uint(w, EbmlId::Range as u64, &*val.v).await?,
                ColourFields::TransferCharacteristics(val) => async_::write_el_uint(w, EbmlId::TransferCharacteristics as u64, &*val.v).await?,
                ColourFields::Primaries(val) => async_::write_el_uint(w, EbmlId::Primaries as u64, &*val.v).await?,
                ColourFields::MaxCll(val) => async_::write_el_uint(w, EbmlId::MaxCll as u64, &*val.v).await?,
                ColourFields::MaxFall(val) => async_::write_el_uint(w, EbmlId::MaxFall as u64, &*val.v).await?,
                ColourFields::StructingMetadata(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl StructingMetadata {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::StructingMetadata as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                StructingMetadataFields::PrimaryRChromaticityX(val) => blocking::write_el_float64(w, EbmlId::PrimaryRChromaticityX as u64, &*val.v)?,
                StructingMetadataFields::PrimaryRChromaticityY(val) => blocking::write_el_float64(w, EbmlId::PrimaryRChromaticityY as u64, &*val.v)?,
                StructingMetadataFields::PrimaryGChromaticityX(val) => blocking::write_el_float64(w, EbmlId::PrimaryGChromaticityX as u64, &*val.v)?,
                StructingMetadataFields::PrimaryGChromaticityY(val) => blocking::write_el_float64(w, EbmlId::PrimaryGChromaticityY as u64, &*val.v)?,
                StructingMetadataFields::PrimaryBChromaticityX(val) => blocking::write_el_float64(w, EbmlId::PrimaryBChromaticityX as u64, &*val.v)?,
                StructingMetadataFields::PrimaryBChromaticityY(val) => blocking::write_el_float64(w, EbmlId::PrimaryBChromaticityY as u64, &*val.v)?,
                StructingMetadataFields::WhitePointChromaticityX(val) => blocking::write_el_float64(w, EbmlId::WhitePointChromaticityX as u64, &*val.v)?,
                StructingMetadataFields::WhitePointChromaticityY(val) => blocking::write_el_float64(w, EbmlId::WhitePointChromaticityY as u64, &*val.v)?,
                StructingMetadataFields::LuminanceMax(val) => blocking::write_el_float64(w, EbmlId::LuminanceMax as u64, &*val.v)?,
                StructingMetadataFields::LuminanceMin(val) => blocking::write_el_float64(w, EbmlId::LuminanceMin as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl StructingMetadata {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::StructingMetadata as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                StructingMetadataFields::PrimaryRChromaticityX(val) => async_::write_el_float64(w, EbmlId::PrimaryRChromaticityX as u64, &*val.v).await?,
                StructingMetadataFields::PrimaryRChromaticityY(val) => async_::write_el_float64(w, EbmlId::PrimaryRChromaticityY as u64, &*val.v).await?,
                StructingMetadataFields::PrimaryGChromaticityX(val) => async_::write_el_float64(w, EbmlId::PrimaryGChromaticityX as u64, &*val.v).await?,
                StructingMetadataFields::PrimaryGChromaticityY(val) => async_::write_el_float64(w, EbmlId::PrimaryGChromaticityY as u64, &*val.v).await?,
                StructingMetadataFields::PrimaryBChromaticityX(val) => async_::write_el_float64(w, EbmlId::PrimaryBChromaticityX as u64, &*val.v).await?,
                StructingMetadataFields::PrimaryBChromaticityY(val) => async_::write_el_float64(w, EbmlId::PrimaryBChromaticityY as u64, &*val.v).await?,
                StructingMetadataFields::WhitePointChromaticityX(val) => async_::write_el_float64(w, EbmlId::WhitePointChromaticityX as u64, &*val.v).await?,
                StructingMetadataFields::WhitePointChromaticityY(val) => async_::write_el_float64(w, EbmlId::WhitePointChromaticityY as u64, &*val.v).await?,
                StructingMetadataFields::LuminanceMax(val) => async_::write_el_float64(w, EbmlId::LuminanceMax as u64, &*val.v).await?,
                StructingMetadataFields::LuminanceMin(val) => async_::write_el_float64(w, EbmlId::LuminanceMin as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl Projection {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::Projection as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ProjectionFields::ProjectionType(val) => blocking::write_el_uint(w, EbmlId::ProjectionType as u64, &*val.v)?,
                ProjectionFields::ProjectionPrivate(val) => blocking::write_el_bin(w, EbmlId::ProjectionPrivate as u64, &val.v)?,
                ProjectionFields::ProjectionPoseYaw(val) => blocking::write_el_float64(w, EbmlId::ProjectionPoseYaw as u64, &*val.v)?,
                ProjectionFields::ProjectionPosePitch(val) => blocking::write_el_float64(w, EbmlId::ProjectionPosePitch as u64, &*val.v)?,
                ProjectionFields::ProjectionPoseRoll(val) => blocking::write_el_float64(w, EbmlId::ProjectionPoseRoll as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl Projection {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::Projection as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ProjectionFields::ProjectionType(val) => async_::write_el_uint(w, EbmlId::ProjectionType as u64, &*val.v).await?,
                ProjectionFields::ProjectionPrivate(val) => async_::write_el_bin(w, EbmlId::ProjectionPrivate as u64, &val.v).await?,
                ProjectionFields::ProjectionPoseYaw(val) => async_::write_el_float64(w, EbmlId::ProjectionPoseYaw as u64, &*val.v).await?,
                ProjectionFields::ProjectionPosePitch(val) => async_::write_el_float64(w, EbmlId::ProjectionPosePitch as u64, &*val.v).await?,
                ProjectionFields::ProjectionPoseRoll(val) => async_::write_el_float64(w, EbmlId::ProjectionPoseRoll as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl Audio {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::Audio as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                AudioFields::SamplingFrequency(val) => blocking::write_el_float64(w, EbmlId::SamplingFrequency as u64, &*val.v)?,
                AudioFields::OutputSamplingFrequency(val) => blocking::write_el_float64(w, EbmlId::OutputSamplingFrequency as u64, &*val.v)?,
                AudioFields::Channels(val) => blocking::write_el_uint(w, EbmlId::Channels as u64, &*val.v)?,
                AudioFields::ChannelPositions(val) => blocking::write_el_bin(w, EbmlId::ChannelPositions as u64, &val.v)?,
                AudioFields::BitDepth(val) => blocking::write_el_uint(w, EbmlId::BitDepth as u64, &*val.v)?,
                AudioFields::Emphasis(val) => blocking::write_el_uint(w, EbmlId::Emphasis as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl Audio {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::Audio as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                AudioFields::SamplingFrequency(val) => async_::write_el_float64(w, EbmlId::SamplingFrequency as u64, &*val.v).await?,
                AudioFields::OutputSamplingFrequency(val) => async_::write_el_float64(w, EbmlId::OutputSamplingFrequency as u64, &*val.v).await?,
                AudioFields::Channels(val) => async_::write_el_uint(w, EbmlId::Channels as u64, &*val.v).await?,
                AudioFields::ChannelPositions(val) => async_::write_el_bin(w, EbmlId::ChannelPositions as u64, &val.v).await?,
                AudioFields::BitDepth(val) => async_::write_el_uint(w, EbmlId::BitDepth as u64, &*val.v).await?,
                AudioFields::Emphasis(val) => async_::write_el_uint(w, EbmlId::Emphasis as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl TrackOperation {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::TrackOperation as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TrackOperationFields::TrackCombinePlanes(val) => val.v.write_blocking(w)?,
                TrackOperationFields::TrackJoinBlocks(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl TrackOperation {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::TrackOperation as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TrackOperationFields::TrackCombinePlanes(val) => val.v.write(w).await?,
                TrackOperationFields::TrackJoinBlocks(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl TrackCombinePlanes {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::TrackCombinePlanes as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TrackCombinePlanesFields::TrackPlane(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl TrackCombinePlanes {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::TrackCombinePlanes as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TrackCombinePlanesFields::TrackPlane(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl TrackPlane {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::TrackPlane as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TrackPlaneFields::TrackPlaneUid(val) => blocking::write_el_uint(w, EbmlId::TrackPlaneUid as u64, &*val.v)?,
                TrackPlaneFields::TrackPlaneType(val) => blocking::write_el_uint(w, EbmlId::TrackPlaneType as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl TrackPlane {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::TrackPlane as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TrackPlaneFields::TrackPlaneUid(val) => async_::write_el_uint(w, EbmlId::TrackPlaneUid as u64, &*val.v).await?,
                TrackPlaneFields::TrackPlaneType(val) => async_::write_el_uint(w, EbmlId::TrackPlaneType as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl TrackJoinBlocks {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::TrackJoinBlocks as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TrackJoinBlocksFields::TrackJoinUid(val) => blocking::write_el_uint(w, EbmlId::TrackJoinUid as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl TrackJoinBlocks {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::TrackJoinBlocks as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TrackJoinBlocksFields::TrackJoinUid(val) => async_::write_el_uint(w, EbmlId::TrackJoinUid as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl ContentEncodings {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::ContentEncodings as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ContentEncodingsFields::ContentEncoding(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl ContentEncodings {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::ContentEncodings as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ContentEncodingsFields::ContentEncoding(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl ContentEncoding {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::ContentEncoding as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ContentEncodingFields::ContentEncodingOrder(val) => blocking::write_el_uint(w, EbmlId::ContentEncodingOrder as u64, &*val.v)?,
                ContentEncodingFields::ContentEncodingScope(val) => blocking::write_el_uint(w, EbmlId::ContentEncodingScope as u64, &*val.v)?,
                ContentEncodingFields::ContentEncodingType(val) => blocking::write_el_uint(w, EbmlId::ContentEncodingType as u64, &*val.v)?,
                ContentEncodingFields::ContentCompression(val) => val.v.write_blocking(w)?,
                ContentEncodingFields::ContentEncryption(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl ContentEncoding {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::ContentEncoding as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ContentEncodingFields::ContentEncodingOrder(val) => async_::write_el_uint(w, EbmlId::ContentEncodingOrder as u64, &*val.v).await?,
                ContentEncodingFields::ContentEncodingScope(val) => async_::write_el_uint(w, EbmlId::ContentEncodingScope as u64, &*val.v).await?,
                ContentEncodingFields::ContentEncodingType(val) => async_::write_el_uint(w, EbmlId::ContentEncodingType as u64, &*val.v).await?,
                ContentEncodingFields::ContentCompression(val) => val.v.write(w).await?,
                ContentEncodingFields::ContentEncryption(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl ContentCompression {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::ContentCompression as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ContentCompressionFields::ContentCompAlgo(val) => blocking::write_el_uint(w, EbmlId::ContentCompAlgo as u64, &*val.v)?,
                ContentCompressionFields::ContentCompSettings(val) => blocking::write_el_bin(w, EbmlId::ContentCompSettings as u64, &val.v)?,
            }
        }
        Ok(size)
    }
}
impl ContentCompression {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::ContentCompression as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ContentCompressionFields::ContentCompAlgo(val) => async_::write_el_uint(w, EbmlId::ContentCompAlgo as u64, &*val.v).await?,
                ContentCompressionFields::ContentCompSettings(val) => async_::write_el_bin(w, EbmlId::ContentCompSettings as u64, &val.v).await?,
            }
        }
        Ok(size)
    }
}

impl ContentEncryption {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::ContentEncryption as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ContentEncryptionFields::ContentEncAlgo(val) => blocking::write_el_uint(w, EbmlId::ContentEncAlgo as u64, &*val.v)?,
                ContentEncryptionFields::ContentEncKeyId(val) => blocking::write_el_bin(w, EbmlId::ContentEncKeyId as u64, &val.v)?,
                ContentEncryptionFields::ContentEncAesSettings(val) => val.v.write_blocking(w)?,
                ContentEncryptionFields::ContentSignature(val) => blocking::write_el_bin(w, EbmlId::ContentSignature as u64, &val.v)?,
                ContentEncryptionFields::ContentSigKeyId(val) => blocking::write_el_bin(w, EbmlId::ContentSigKeyId as u64, &val.v)?,
                ContentEncryptionFields::ContentSigAlgo(val) => blocking::write_el_uint(w, EbmlId::ContentSigAlgo as u64, &*val.v)?,
                ContentEncryptionFields::ContentSigHashAlgo(val) => blocking::write_el_uint(w, EbmlId::ContentSigHashAlgo as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl ContentEncryption {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::ContentEncryption as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ContentEncryptionFields::ContentEncAlgo(val) => async_::write_el_uint(w, EbmlId::ContentEncAlgo as u64, &*val.v).await?,
                ContentEncryptionFields::ContentEncKeyId(val) => async_::write_el_bin(w, EbmlId::ContentEncKeyId as u64, &val.v).await?,
                ContentEncryptionFields::ContentEncAesSettings(val) => val.v.write(w).await?,
                ContentEncryptionFields::ContentSignature(val) => async_::write_el_bin(w, EbmlId::ContentSignature as u64, &val.v).await?,
                ContentEncryptionFields::ContentSigKeyId(val) => async_::write_el_bin(w, EbmlId::ContentSigKeyId as u64, &val.v).await?,
                ContentEncryptionFields::ContentSigAlgo(val) => async_::write_el_uint(w, EbmlId::ContentSigAlgo as u64, &*val.v).await?,
                ContentEncryptionFields::ContentSigHashAlgo(val) => async_::write_el_uint(w, EbmlId::ContentSigHashAlgo as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl ContentEncAesSettings {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::ContentEncAesSettings as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ContentEncAesSettingsFields::AesSettingsCipherMode(val) => blocking::write_el_uint(w, EbmlId::AesSettingsCipherMode as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl ContentEncAesSettings {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::ContentEncAesSettings as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ContentEncAesSettingsFields::AesSettingsCipherMode(val) => async_::write_el_uint(w, EbmlId::AesSettingsCipherMode as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl Cues {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::Cues as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                CuesFields::CuePoint(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl Cues {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::Cues as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                CuesFields::CuePoint(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl CuePoint {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::CuePoint as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                CuePointFields::CueTime(val) => blocking::write_el_uint(w, EbmlId::CueTime as u64, &*val.v)?,
                CuePointFields::CueTrackPositions(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl CuePoint {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::CuePoint as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                CuePointFields::CueTime(val) => async_::write_el_uint(w, EbmlId::CueTime as u64, &*val.v).await?,
                CuePointFields::CueTrackPositions(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl CueTrackPositions {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::CueTrackPositions as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                CueTrackPositionsFields::CueTrack(val) => blocking::write_el_uint(w, EbmlId::CueTrack as u64, &*val.v)?,
                CueTrackPositionsFields::CueClusterPosition(val) => blocking::write_el_uint(w, EbmlId::CueClusterPosition as u64, &*val.v)?,
                CueTrackPositionsFields::CueRelativePosition(val) => blocking::write_el_uint(w, EbmlId::CueRelativePosition as u64, &*val.v)?,
                CueTrackPositionsFields::CueDuration(val) => blocking::write_el_uint(w, EbmlId::CueDuration as u64, &*val.v)?,
                CueTrackPositionsFields::CueBlockNumber(val) => blocking::write_el_uint(w, EbmlId::CueBlockNumber as u64, &*val.v)?,
                CueTrackPositionsFields::CueCodecState(val) => blocking::write_el_uint(w, EbmlId::CueCodecState as u64, &*val.v)?,
                CueTrackPositionsFields::CueReference(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl CueTrackPositions {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::CueTrackPositions as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                CueTrackPositionsFields::CueTrack(val) => async_::write_el_uint(w, EbmlId::CueTrack as u64, &*val.v).await?,
                CueTrackPositionsFields::CueClusterPosition(val) => async_::write_el_uint(w, EbmlId::CueClusterPosition as u64, &*val.v).await?,
                CueTrackPositionsFields::CueRelativePosition(val) => async_::write_el_uint(w, EbmlId::CueRelativePosition as u64, &*val.v).await?,
                CueTrackPositionsFields::CueDuration(val) => async_::write_el_uint(w, EbmlId::CueDuration as u64, &*val.v).await?,
                CueTrackPositionsFields::CueBlockNumber(val) => async_::write_el_uint(w, EbmlId::CueBlockNumber as u64, &*val.v).await?,
                CueTrackPositionsFields::CueCodecState(val) => async_::write_el_uint(w, EbmlId::CueCodecState as u64, &*val.v).await?,
                CueTrackPositionsFields::CueReference(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl CueReference {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::CueReference as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                CueReferenceFields::CueRefTime(val) => blocking::write_el_uint(w, EbmlId::CueRefTime as u64, &*val.v)?,
                CueReferenceFields::CueRefCluster(val) => blocking::write_el_uint(w, EbmlId::CueRefCluster as u64, &*val.v)?,
                CueReferenceFields::CueRefNumber(val) => blocking::write_el_uint(w, EbmlId::CueRefNumber as u64, &*val.v)?,
                CueReferenceFields::CueRefCodecState(val) => blocking::write_el_uint(w, EbmlId::CueRefCodecState as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl CueReference {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::CueReference as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                CueReferenceFields::CueRefTime(val) => async_::write_el_uint(w, EbmlId::CueRefTime as u64, &*val.v).await?,
                CueReferenceFields::CueRefCluster(val) => async_::write_el_uint(w, EbmlId::CueRefCluster as u64, &*val.v).await?,
                CueReferenceFields::CueRefNumber(val) => async_::write_el_uint(w, EbmlId::CueRefNumber as u64, &*val.v).await?,
                CueReferenceFields::CueRefCodecState(val) => async_::write_el_uint(w, EbmlId::CueRefCodecState as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl Attachments {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::Attachments as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                AttachmentsFields::AttachedFile(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl Attachments {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::Attachments as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                AttachmentsFields::AttachedFile(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl AttachedFile {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::AttachedFile as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                AttachedFileFields::FileDescription(val) => blocking::write_el_utf8(w, EbmlId::FileDescription as u64, &val.v)?,
                AttachedFileFields::FileName(val) => blocking::write_el_utf8(w, EbmlId::FileName as u64, &val.v)?,
                AttachedFileFields::FileMediaType(val) => blocking::write_el_string(w, EbmlId::FileMediaType as u64, &val.v)?,
                AttachedFileFields::FileData(val) => blocking::write_el_bin(w, EbmlId::FileData as u64, &val.v)?,
                AttachedFileFields::FileUid(val) => blocking::write_el_uint(w, EbmlId::FileUid as u64, &*val.v)?,
                AttachedFileFields::FileReferral(val) => blocking::write_el_bin(w, EbmlId::FileReferral as u64, &val.v)?,
                AttachedFileFields::FileUsedStartTime(val) => blocking::write_el_uint(w, EbmlId::FileUsedStartTime as u64, &*val.v)?,
                AttachedFileFields::FileUsedEndTime(val) => blocking::write_el_uint(w, EbmlId::FileUsedEndTime as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl AttachedFile {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::AttachedFile as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                AttachedFileFields::FileDescription(val) => async_::write_el_utf8(w, EbmlId::FileDescription as u64, &val.v).await?,
                AttachedFileFields::FileName(val) => async_::write_el_utf8(w, EbmlId::FileName as u64, &val.v).await?,
                AttachedFileFields::FileMediaType(val) => async_::write_el_string(w, EbmlId::FileMediaType as u64, &val.v).await?,
                AttachedFileFields::FileData(val) => async_::write_el_bin(w, EbmlId::FileData as u64, &val.v).await?,
                AttachedFileFields::FileUid(val) => async_::write_el_uint(w, EbmlId::FileUid as u64, &*val.v).await?,
                AttachedFileFields::FileReferral(val) => async_::write_el_bin(w, EbmlId::FileReferral as u64, &val.v).await?,
                AttachedFileFields::FileUsedStartTime(val) => async_::write_el_uint(w, EbmlId::FileUsedStartTime as u64, &*val.v).await?,
                AttachedFileFields::FileUsedEndTime(val) => async_::write_el_uint(w, EbmlId::FileUsedEndTime as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl Chapters {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::Chapters as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ChaptersFields::EditionEntry(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl Chapters {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::Chapters as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ChaptersFields::EditionEntry(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl EditionEntry {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::EditionEntry as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                EditionEntryFields::EditionUid(val) => blocking::write_el_uint(w, EbmlId::EditionUid as u64, &*val.v)?,
                EditionEntryFields::EditionFlagHidden(val) => blocking::write_el_uint(w, EbmlId::EditionFlagHidden as u64, &*val.v)?,
                EditionEntryFields::EditionFlagDefault(val) => blocking::write_el_uint(w, EbmlId::EditionFlagDefault as u64, &*val.v)?,
                EditionEntryFields::EditionFlagOrdered(val) => blocking::write_el_uint(w, EbmlId::EditionFlagOrdered as u64, &*val.v)?,
                EditionEntryFields::EditionDisplay(val) => val.v.write_blocking(w)?,
                EditionEntryFields::ChapterAtom(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl EditionEntry {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::EditionEntry as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                EditionEntryFields::EditionUid(val) => async_::write_el_uint(w, EbmlId::EditionUid as u64, &*val.v).await?,
                EditionEntryFields::EditionFlagHidden(val) => async_::write_el_uint(w, EbmlId::EditionFlagHidden as u64, &*val.v).await?,
                EditionEntryFields::EditionFlagDefault(val) => async_::write_el_uint(w, EbmlId::EditionFlagDefault as u64, &*val.v).await?,
                EditionEntryFields::EditionFlagOrdered(val) => async_::write_el_uint(w, EbmlId::EditionFlagOrdered as u64, &*val.v).await?,
                EditionEntryFields::EditionDisplay(val) => val.v.write(w).await?,
                EditionEntryFields::ChapterAtom(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl EditionDisplay {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::EditionDisplay as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                EditionDisplayFields::EditionString(val) => blocking::write_el_utf8(w, EbmlId::EditionString as u64, &val.v)?,
                EditionDisplayFields::EditionLanguageIetf(val) => blocking::write_el_string(w, EbmlId::EditionLanguageIetf as u64, &val.v)?,
            }
        }
        Ok(size)
    }
}
impl EditionDisplay {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::EditionDisplay as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                EditionDisplayFields::EditionString(val) => async_::write_el_utf8(w, EbmlId::EditionString as u64, &val.v).await?,
                EditionDisplayFields::EditionLanguageIetf(val) => async_::write_el_string(w, EbmlId::EditionLanguageIetf as u64, &val.v).await?,
            }
        }
        Ok(size)
    }
}

impl ChapterAtom {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::ChapterAtom as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ChapterAtomFields::ChapterUid(val) => blocking::write_el_uint(w, EbmlId::ChapterUid as u64, &*val.v)?,
                ChapterAtomFields::ChapterStringUid(val) => blocking::write_el_utf8(w, EbmlId::ChapterStringUid as u64, &val.v)?,
                ChapterAtomFields::ChapterTimeStart(val) => blocking::write_el_uint(w, EbmlId::ChapterTimeStart as u64, &*val.v)?,
                ChapterAtomFields::ChapterTimeEnd(val) => blocking::write_el_uint(w, EbmlId::ChapterTimeEnd as u64, &*val.v)?,
                ChapterAtomFields::ChapterFlagHidden(val) => blocking::write_el_uint(w, EbmlId::ChapterFlagHidden as u64, &*val.v)?,
                ChapterAtomFields::ChapterFlagEnabled(val) => blocking::write_el_uint(w, EbmlId::ChapterFlagEnabled as u64, &*val.v)?,
                ChapterAtomFields::ChapterSegmentUuid(val) => blocking::write_el_bin(w, EbmlId::ChapterSegmentUuid as u64, &val.v)?,
                ChapterAtomFields::ChapterSkipType(val) => blocking::write_el_uint(w, EbmlId::ChapterSkipType as u64, &*val.v)?,
                ChapterAtomFields::ChapterSegmentEditionUid(val) => blocking::write_el_uint(w, EbmlId::ChapterSegmentEditionUid as u64, &*val.v)?,
                ChapterAtomFields::ChapterPhysicalEquiv(val) => blocking::write_el_uint(w, EbmlId::ChapterPhysicalEquiv as u64, &*val.v)?,
                ChapterAtomFields::ChapterTrack(val) => val.v.write_blocking(w)?,
                ChapterAtomFields::ChapterDisplay(val) => val.v.write_blocking(w)?,
                ChapterAtomFields::ChapProcess(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl ChapterAtom {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::ChapterAtom as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ChapterAtomFields::ChapterUid(val) => async_::write_el_uint(w, EbmlId::ChapterUid as u64, &*val.v).await?,
                ChapterAtomFields::ChapterStringUid(val) => async_::write_el_utf8(w, EbmlId::ChapterStringUid as u64, &val.v).await?,
                ChapterAtomFields::ChapterTimeStart(val) => async_::write_el_uint(w, EbmlId::ChapterTimeStart as u64, &*val.v).await?,
                ChapterAtomFields::ChapterTimeEnd(val) => async_::write_el_uint(w, EbmlId::ChapterTimeEnd as u64, &*val.v).await?,
                ChapterAtomFields::ChapterFlagHidden(val) => async_::write_el_uint(w, EbmlId::ChapterFlagHidden as u64, &*val.v).await?,
                ChapterAtomFields::ChapterFlagEnabled(val) => async_::write_el_uint(w, EbmlId::ChapterFlagEnabled as u64, &*val.v).await?,
                ChapterAtomFields::ChapterSegmentUuid(val) => async_::write_el_bin(w, EbmlId::ChapterSegmentUuid as u64, &val.v).await?,
                ChapterAtomFields::ChapterSkipType(val) => async_::write_el_uint(w, EbmlId::ChapterSkipType as u64, &*val.v).await?,
                ChapterAtomFields::ChapterSegmentEditionUid(val) => async_::write_el_uint(w, EbmlId::ChapterSegmentEditionUid as u64, &*val.v).await?,
                ChapterAtomFields::ChapterPhysicalEquiv(val) => async_::write_el_uint(w, EbmlId::ChapterPhysicalEquiv as u64, &*val.v).await?,
                ChapterAtomFields::ChapterTrack(val) => val.v.write(w).await?,
                ChapterAtomFields::ChapterDisplay(val) => val.v.write(w).await?,
                ChapterAtomFields::ChapProcess(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl ChapterTrack {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::ChapterTrack as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ChapterTrackFields::ChapterTrackUid(val) => blocking::write_el_uint(w, EbmlId::ChapterTrackUid as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl ChapterTrack {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::ChapterTrack as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ChapterTrackFields::ChapterTrackUid(val) => async_::write_el_uint(w, EbmlId::ChapterTrackUid as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl ChapterDisplay {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::ChapterDisplay as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ChapterDisplayFields::ChapString(val) => blocking::write_el_utf8(w, EbmlId::ChapString as u64, &val.v)?,
                ChapterDisplayFields::ChapLanguage(val) => blocking::write_el_string(w, EbmlId::ChapLanguage as u64, &val.v)?,
                ChapterDisplayFields::ChapLanguageBcp47(val) => blocking::write_el_string(w, EbmlId::ChapLanguageBcp47 as u64, &val.v)?,
                ChapterDisplayFields::ChapCountry(val) => blocking::write_el_string(w, EbmlId::ChapCountry as u64, &val.v)?,
            }
        }
        Ok(size)
    }
}
impl ChapterDisplay {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::ChapterDisplay as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ChapterDisplayFields::ChapString(val) => async_::write_el_utf8(w, EbmlId::ChapString as u64, &val.v).await?,
                ChapterDisplayFields::ChapLanguage(val) => async_::write_el_string(w, EbmlId::ChapLanguage as u64, &val.v).await?,
                ChapterDisplayFields::ChapLanguageBcp47(val) => async_::write_el_string(w, EbmlId::ChapLanguageBcp47 as u64, &val.v).await?,
                ChapterDisplayFields::ChapCountry(val) => async_::write_el_string(w, EbmlId::ChapCountry as u64, &val.v).await?,
            }
        }
        Ok(size)
    }
}

impl ChapProcess {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::ChapProcess as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ChapProcessFields::ChapProcessCodecId(val) => blocking::write_el_uint(w, EbmlId::ChapProcessCodecId as u64, &*val.v)?,
                ChapProcessFields::ChapProcessPrivate(val) => blocking::write_el_bin(w, EbmlId::ChapProcessPrivate as u64, &val.v)?,
                ChapProcessFields::ChapProcessCommand(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl ChapProcess {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::ChapProcess as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ChapProcessFields::ChapProcessCodecId(val) => async_::write_el_uint(w, EbmlId::ChapProcessCodecId as u64, &*val.v).await?,
                ChapProcessFields::ChapProcessPrivate(val) => async_::write_el_bin(w, EbmlId::ChapProcessPrivate as u64, &val.v).await?,
                ChapProcessFields::ChapProcessCommand(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl ChapProcessCommand {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::ChapProcessCommand as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ChapProcessCommandFields::ChapProcessTime(val) => blocking::write_el_uint(w, EbmlId::ChapProcessTime as u64, &*val.v)?,
                ChapProcessCommandFields::ChapProcessData(val) => blocking::write_el_bin(w, EbmlId::ChapProcessData as u64, &val.v)?,
            }
        }
        Ok(size)
    }
}
impl ChapProcessCommand {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::ChapProcessCommand as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                ChapProcessCommandFields::ChapProcessTime(val) => async_::write_el_uint(w, EbmlId::ChapProcessTime as u64, &*val.v).await?,
                ChapProcessCommandFields::ChapProcessData(val) => async_::write_el_bin(w, EbmlId::ChapProcessData as u64, &val.v).await?,
            }
        }
        Ok(size)
    }
}

impl Tags {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::Tags as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TagsFields::Tag(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl Tags {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::Tags as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TagsFields::Tag(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl Tag {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::Tag as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TagFields::Targets(val) => val.v.write_blocking(w)?,
                TagFields::SimpleTag(val) => val.v.write_blocking(w)?,
            }
        }
        Ok(size)
    }
}
impl Tag {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::Tag as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TagFields::Targets(val) => val.v.write(w).await?,
                TagFields::SimpleTag(val) => val.v.write(w).await?,
            }
        }
        Ok(size)
    }
}

impl Targets {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::Targets as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TargetsFields::TargetTypeValue(val) => blocking::write_el_uint(w, EbmlId::TargetTypeValue as u64, &*val.v)?,
                TargetsFields::TargetType(val) => blocking::write_el_string(w, EbmlId::TargetType as u64, &val.v)?,
                TargetsFields::TagTrackUid(val) => blocking::write_el_uint(w, EbmlId::TagTrackUid as u64, &*val.v)?,
                TargetsFields::TagEditionUid(val) => blocking::write_el_uint(w, EbmlId::TagEditionUid as u64, &*val.v)?,
                TargetsFields::TagChapterUid(val) => blocking::write_el_uint(w, EbmlId::TagChapterUid as u64, &*val.v)?,
                TargetsFields::TagAttachmentUid(val) => blocking::write_el_uint(w, EbmlId::TagAttachmentUid as u64, &*val.v)?,
            }
        }
        Ok(size)
    }
}
impl Targets {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::Targets as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                TargetsFields::TargetTypeValue(val) => async_::write_el_uint(w, EbmlId::TargetTypeValue as u64, &*val.v).await?,
                TargetsFields::TargetType(val) => async_::write_el_string(w, EbmlId::TargetType as u64, &val.v).await?,
                TargetsFields::TagTrackUid(val) => async_::write_el_uint(w, EbmlId::TagTrackUid as u64, &*val.v).await?,
                TargetsFields::TagEditionUid(val) => async_::write_el_uint(w, EbmlId::TagEditionUid as u64, &*val.v).await?,
                TargetsFields::TagChapterUid(val) => async_::write_el_uint(w, EbmlId::TagChapterUid as u64, &*val.v).await?,
                TargetsFields::TagAttachmentUid(val) => async_::write_el_uint(w, EbmlId::TagAttachmentUid as u64, &*val.v).await?,
            }
        }
        Ok(size)
    }
}

impl SimpleTag {
    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body_blocking(&mut buf)?;
        let mut size = self.write_header_blocking(w, buf.len() as u64)?;
        w.write_all(&buf)?; size += buf.len();
        Ok(size)
    }
    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(blocking::write_element_id_size(w, EbmlId::SimpleTag as u64, size)?)
    }
    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                SimpleTagFields::TagName(val) => blocking::write_el_utf8(w, EbmlId::TagName as u64, &val.v)?,
                SimpleTagFields::TagLanguage(val) => blocking::write_el_string(w, EbmlId::TagLanguage as u64, &val.v)?,
                SimpleTagFields::TagLanguageBcp47(val) => blocking::write_el_string(w, EbmlId::TagLanguageBcp47 as u64, &val.v)?,
                SimpleTagFields::TagDefault(val) => blocking::write_el_uint(w, EbmlId::TagDefault as u64, &*val.v)?,
                SimpleTagFields::TagDefaultBogus(val) => blocking::write_el_uint(w, EbmlId::TagDefaultBogus as u64, &*val.v)?,
                SimpleTagFields::TagString(val) => blocking::write_el_utf8(w, EbmlId::TagString as u64, &val.v)?,
                SimpleTagFields::TagBinary(val) => blocking::write_el_bin(w, EbmlId::TagBinary as u64, &val.v)?,
            }
        }
        Ok(size)
    }
}
impl SimpleTag {
    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut buf = vec![]; self.write_body(&mut buf).await?;
        let mut size = self.write_header(w, buf.len() as u64).await?;
        w.write_all(&buf).await?; size += buf.len();
        Ok(size)
    }
    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {
        Ok(async_::write_element_id_size(w, EbmlId::SimpleTag as u64, size).await?)
    }
    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {
        let mut size = 0usize;
        for el in self.elements() {
            size += match el {
                SimpleTagFields::TagName(val) => async_::write_el_utf8(w, EbmlId::TagName as u64, &val.v).await?,
                SimpleTagFields::TagLanguage(val) => async_::write_el_string(w, EbmlId::TagLanguage as u64, &val.v).await?,
                SimpleTagFields::TagLanguageBcp47(val) => async_::write_el_string(w, EbmlId::TagLanguageBcp47 as u64, &val.v).await?,
                SimpleTagFields::TagDefault(val) => async_::write_el_uint(w, EbmlId::TagDefault as u64, &*val.v).await?,
                SimpleTagFields::TagDefaultBogus(val) => async_::write_el_uint(w, EbmlId::TagDefaultBogus as u64, &*val.v).await?,
                SimpleTagFields::TagString(val) => async_::write_el_utf8(w, EbmlId::TagString as u64, &val.v).await?,
                SimpleTagFields::TagBinary(val) => async_::write_el_bin(w, EbmlId::TagBinary as u64, &val.v).await?,
            }
        }
        Ok(size)
    }
}

