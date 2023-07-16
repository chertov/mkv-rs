use byteorder::{BigEndian, WriteBytesExt};
use time::OffsetDateTime;

use super::MatroskaError;
use super::ids::EbmlId;
use super::ElementSize;

// https://www.rfc-editor.org/rfc/rfc8794.html#name-variable-size-integer
// VINT_WIDTH - zero or more bits value 0 followed by VINT_MARKER.
// VINT_MARKER - The single bit value 1 followed by VINT_DATA. The VINT_MARKER serves as a separator between the VINT_WIDTH and VINT_DATA.
//                         VINT_WIDTH    VINT_MARKER     VINT_DATA   2           3           4           5           6           7
// BINARY VINT:       0b___0000_000______1_______________1111_1111___1111_1111___1111_1111___1111_1111___1111_1111___1111_1111___1111_1111
// u64 VINT MAX value 0b___0000_000______0_______________1111_1111___1111_1111___1111_1111___1111_1111___1111_1111___1111_1111___1111_1111
const VINT_MAX: u64 = VINT_MAX_FOR_8_BYTES;
const VINT_MAX_FOR_1_BYTES: u64 = 2u64.pow(7 /* bits in first byte */ + 0 /* data bytes */ * 8 /* bits in byte */) - 1;
const VINT_MAX_FOR_2_BYTES: u64 = 2u64.pow(6 /* bits in first byte */ + 1 /* data bytes */ * 8 /* bits in byte */) - 1;
const VINT_MAX_FOR_3_BYTES: u64 = 2u64.pow(5 /* bits in first byte */ + 2 /* data bytes */ * 8 /* bits in byte */) - 1;
const VINT_MAX_FOR_4_BYTES: u64 = 2u64.pow(4 /* bits in first byte */ + 3 /* data bytes */ * 8 /* bits in byte */) - 1;
const VINT_MAX_FOR_5_BYTES: u64 = 2u64.pow(3 /* bits in first byte */ + 4 /* data bytes */ * 8 /* bits in byte */) - 1;
const VINT_MAX_FOR_6_BYTES: u64 = 2u64.pow(2 /* bits in first byte */ + 5 /* data bytes */ * 8 /* bits in byte */) - 1;
const VINT_MAX_FOR_7_BYTES: u64 = 2u64.pow(1 /* bits in first byte */ + 6 /* data bytes */ * 8 /* bits in byte */) - 1;
const VINT_MAX_FOR_8_BYTES: u64 = 2u64.pow(0 /* bits in first byte */ + 7 /* data bytes */ * 8 /* bits in byte */) - 1;

pub const SIZE_UNKNOWN: u64 = VINT_MAX_FOR_1_BYTES;

macro_rules! define_read_element_id_size {
    ($($impl_async:ident)?) => {
        $(#[async_blocking::$impl_async])?
        pub fn read_element_id_size<R: std::io::Read>(reader: &mut R) -> Result<(EbmlId, ElementSize, u64), MatroskaError> {
            let (id, id_len) = read_element_id(reader).map(|await_|await_)?;
            let id = EbmlId::from_u64(id).map_err(|err| MatroskaError::InvalidID(id))?;
            let (size, size_len) = read_element_size(reader).map(|await_|await_)?;

            let unknown_size = match size {
                ElementSize::Sized(_) => { return Ok((id, size, id_len + size_len)); },
                ElementSize::Unknown(unknown_size) => unknown_size
            };
            if id.unknown_size_allowed() {
                Ok((id, ElementSize::Unknown(unknown_size), id_len + size_len))
            } else {
                Err(MatroskaError::InvalidSize(id, size))
            }
        }
    }
}

macro_rules! define_read_vint {
    ($($impl_async:ident)?) => {
        $(#[async_blocking::$impl_async])?
        pub fn read_vint<R: std::io::Read>(r: &mut R) -> Result<(/* vint value */ u64, /* len */ u64), MatroskaError> {
            // https://www.rfc-editor.org/rfc/rfc8794.html#name-variable-size-integer

            // VINT_WIDTH - zero or more bits value 0 followed by VINT_MARKER.
            // VINT_MARKER - The single bit value 1 followed by VINT_DATA. The VINT_MARKER serves as a separator between the VINT_WIDTH and VINT_DATA.
            // VINT_DATA
            // | VINT_WIDTH | VINT_MARKER | VINT_DATA

            let first_byte = r.read_u8().map(|await_|await_)?;
            let mask: u8;
            let more_bytes: u64;
            if      first_byte & 0b_1000_0000 != 0 { mask = 0b_0111_1111; more_bytes = 0 }
            else if first_byte & 0b_0100_0000 != 0 { mask = 0b_0011_1111; more_bytes = 1 }
            else if first_byte & 0b_0010_0000 != 0 { mask = 0b_0001_1111; more_bytes = 2 }
            else if first_byte & 0b_0001_0000 != 0 { mask = 0b_0000_1111; more_bytes = 3 }
            else if first_byte & 0b_0000_1000 != 0 { mask = 0b_0000_0111; more_bytes = 4 }
            else if first_byte & 0b_0000_0100 != 0 { mask = 0b_0000_0011; more_bytes = 5 }
            else if first_byte & 0b_0000_0010 != 0 { mask = 0b_0000_0001; more_bytes = 6 }
            else if first_byte & 0b_0000_0001 != 0 { mask = 0b_0000_0000; more_bytes = 7 }
            else {
                return Err(MatroskaError::InvalidVarInt);
            }

            let mut vint = (first_byte & mask) as u64;
            for _ in 0..more_bytes {
                vint <<= 8;
                let next_byte = r.read_u8().map(|await_|await_)?;
                vint += next_byte as u64;
            }
            Ok((vint, more_bytes as u64 + 1))
        }
    }
}


macro_rules! define_read_element_id {
    ($($impl_async:ident)?) => {
        $(#[async_blocking::$impl_async])?
        pub fn read_element_id<R: std::io::Read>(r: &mut R) -> Result<(/* vint value */ u64, /* len */ u64), MatroskaError> {
            let (id, id_len) = read_vint(r).map(|await_|await_)?;

            // https://www.rfc-editor.org/rfc/rfc8794.html#tableElementIDValidity
            // Check Element ID VINT_DATA for all 0 or 1 bits
            if id == 0
                || id == VINT_MAX_FOR_1_BYTES
                || id == VINT_MAX_FOR_2_BYTES
                || id == VINT_MAX_FOR_3_BYTES
                || id == VINT_MAX_FOR_4_BYTES
                || id == VINT_MAX_FOR_5_BYTES
                || id == VINT_MAX_FOR_6_BYTES
                || id == VINT_MAX_FOR_7_BYTES
                || id == VINT_MAX_FOR_8_BYTES
            {
                return Err(MatroskaError::InvalidID(id));
            }

            // Element ID uses VINT_MARKER as a value bit, we need to set it to 1
            let marker_pos = (8 - id_len) /* bits in first byte */ + (id_len - 1) /* data bytes */ * 8 /* bits in byte */;
            let id = id + (1u64 << marker_pos);

            Ok((id, id_len))
        }
    }
}

macro_rules! define_read_element_size {
    ($($impl_async:ident)?) => {
        $(#[async_blocking::$impl_async])?
        pub fn read_element_size<R: std::io::Read>(r: &mut R) -> Result<(ElementSize, u64), MatroskaError> {
            let (val, len) = read_vint(r).map(|await_|await_)?;
            if     (val == VINT_MAX_FOR_1_BYTES && len == 1)
                || (val == VINT_MAX_FOR_2_BYTES && len == 2)
                || (val == VINT_MAX_FOR_3_BYTES && len == 3)
                || (val == VINT_MAX_FOR_4_BYTES && len == 4)
                || (val == VINT_MAX_FOR_5_BYTES && len == 5)
                || (val == VINT_MAX_FOR_6_BYTES && len == 6)
                || (val == VINT_MAX_FOR_7_BYTES && len == 7)
                || (val == VINT_MAX_FOR_8_BYTES && len == 8)
            {
                return Ok((ElementSize::Unknown(val), len));
            }
            Ok((ElementSize::Sized(val), len))
        }
    }
}

macro_rules! define_read_uint {
    ($($impl_async:ident)?) => {
        $(#[async_blocking::$impl_async])?
        pub fn read_uint<R: std::io::Read>(r: &mut R, size: u64) -> Result<u64, MatroskaError> {
            Ok(match size {
                s @ 0..=8 => {
                    let mut val = 0u64;
                    for _ in 0..s {
                        val = val << 8;
                        val += r.read_u8().map(|await_|await_)? as u64;
                    }
                    val
                },
                invalid_size => {
                    error!("read_uint invalid_size {}", invalid_size);
                    return Err(MatroskaError::InvalidUint);
                },
            })
        }
    }
}

macro_rules! define_read_int {
    ($($impl_async:ident)?) => {
        $(#[async_blocking::$impl_async])?
        pub fn read_int<R: std::io::Read>(r: &mut R, size: u64) -> Result<i64, MatroskaError> {
            Ok(match size {
                s @ 0..=8 => {
                    let mut val = 0i64;
                    for _ in 0..s {
                        val = val << 8;
                        val += r.read_u8().map(|await_|await_)? as i64;
                    }
                    val
                }
                invalid_size => {
                    error!("read_int invalid_size {}", invalid_size);
                    return Err(MatroskaError::InvalidUint);
                },
            })
        }
    }
}

macro_rules! define_read_float {
    () => {
        pub fn read_float<R: std::io::Read>(r: &mut R, size: u64) -> Result<f64, MatroskaError> {
            Ok(match size {
                4 => r.read_f32::<BigEndian>()? as f64,
                8 => r.read_f64::<BigEndian>()? as f64,
                invalid_size => {
                    error!("read_float invalid_size {}", invalid_size);
                    return Err(MatroskaError::InvalidFloat);
                },
            })
        }
    };
    (impl_async) => {
        #[async_blocking::impl_async]
        pub fn read_float<R: std::io::Read>(r: &mut R, size: u64) -> Result<f64, MatroskaError> {
            Ok(match size {
                4 => r.read_f32().await? as f64,
                8 => r.read_f64().await? as f64,
                invalid_size => {
                    error!("read_float invalid_size {}", invalid_size);
                    return Err(MatroskaError::InvalidFloat);
                },
            })
        }
    }
}

macro_rules! define_read_date {
    ($($impl_async:ident)?) => {
        $(#[async_blocking::$impl_async])?
        pub fn read_date<R: std::io::Read>(r: &mut R, size: u64) -> Result<OffsetDateTime, MatroskaError> {
            if size == 8 {
                use time::macros::datetime;
                let res = read_int(r, size).map(|await_|await_);
                res.map(|d| datetime!(2001-01-01 00:00:00 UTC) + time::Duration::nanoseconds(d))
            } else {
                Ok(Err(MatroskaError::InvalidDate)?)
            }
        }
    }
}

pub use io::*;
mod io {
    use super::{MatroskaError, ElementSize, EbmlId,
                VINT_MAX_FOR_1_BYTES,
                VINT_MAX_FOR_2_BYTES,
                VINT_MAX_FOR_3_BYTES,
                VINT_MAX_FOR_4_BYTES,
                VINT_MAX_FOR_5_BYTES,
                VINT_MAX_FOR_6_BYTES,
                VINT_MAX_FOR_7_BYTES,
                VINT_MAX_FOR_8_BYTES, };

    pub mod blocking {
        use super::*;
        use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
        use time::OffsetDateTime;

        define_read_element_id_size!();
        define_read_vint!();
        define_read_element_id!();
        define_read_element_size!();

        define_read_uint!();
        define_read_int!();
        define_read_float!();

        define_read_date!();

        pub fn read_string<R: std::io::Read>(r: &mut R, size: u64) -> Result<String, MatroskaError> {
            /*FIXME - limit this to ASCII set*/
            let bytes = read_bin(r, size).map(|await_| await_)?;
            Ok(String::from_utf8(bytes).map_err(MatroskaError::UTF8)?)
        }

        pub fn read_utf8<R: std::io::Read>(r: &mut R, size: u64) -> Result<String, MatroskaError> {
            let bytes = read_bin(r, size)?;
            Ok(String::from_utf8(bytes).map_err(MatroskaError::UTF8)?)
        }

        pub fn read_bin<R: std::io::Read>(r: &mut R, size: u64) -> Result<Vec<u8>, MatroskaError> {
            let mut buf = vec![0; size as usize];
            r.read_exact(&mut buf).map_err(MatroskaError::Io)?;
            Ok(buf)
        }

        pub fn write_element_id_size<W: std::io::Write>(w: &mut W, id: u64, size: u64) -> Result<usize, MatroskaError> {
            let buf = gen_element_id_size(id, size);
            w.write_all(&buf)?;
            Ok(buf.len())
        }
        pub fn write_el_uint<W: std::io::Write>(w: &mut W, id: u64, val: &u64) -> Result<usize, MatroskaError> {
            let buf = gen_uint(*val);
            let header_buf = gen_element_id_size(id, buf.len() as u64);
            w.write_all(&header_buf)?;
            w.write_all(&buf)?;
            Ok(header_buf.len() + buf.len())
        }
        pub fn write_el_int<W: std::io::Write>(w: &mut W, id: u64, val: &i64) -> Result<usize, MatroskaError> {
            let buf = gen_int(*val);
            let header_buf = gen_element_id_size(id, buf.len() as u64);
            w.write_all(&header_buf)?;
            w.write_all(&buf)?;
            Ok(header_buf.len() + buf.len())
        }
        pub fn write_el_float32<W: std::io::Write>(w: &mut W, id: u64, val: &f32) -> Result<usize, MatroskaError> {
            let mut buf = vec![];
            buf.write_f32::<BigEndian>(*val)?;
            let header_buf = gen_element_id_size(id, buf.len() as u64);
            w.write_all(&header_buf)?;
            w.write_all(&buf)?;
            Ok(header_buf.len() + buf.len())
        }
        pub fn write_el_float64<W: std::io::Write>(w: &mut W, id: u64, val: &f64) -> Result<usize, MatroskaError> {
            let mut buf = vec![];
            buf.write_f64::<BigEndian>(*val)?;
            let header_buf = gen_element_id_size(id, buf.len() as u64);
            w.write_all(&header_buf)?;
            w.write_all(&buf)?;
            Ok(header_buf.len() + buf.len())
        }
        pub fn write_el_string<W: std::io::Write>(w: &mut W, id: u64, str: &str) -> Result<usize, MatroskaError> {
            write_el_bin(w, id, str.as_bytes())
        }
        pub fn write_el_utf8<W: std::io::Write>(w: &mut W, id: u64, str: &str) -> Result<usize, MatroskaError> {
            write_el_bin(w, id, str.as_bytes())
        }
        pub fn write_el_bin<W: std::io::Write>(w: &mut W, id: u64, buf: &[u8]) -> Result<usize, MatroskaError> {
            let header_buf = gen_element_id_size(id, buf.len() as u64);
            w.write_all(&header_buf)?;
            w.write_all(buf)?;
            Ok(header_buf.len() + buf.len())
        }
        pub fn write_el_date<W: std::io::Write>(w: &mut W, id: u64, val: &OffsetDateTime) -> Result<usize, MatroskaError> {
            let unix_ts = val.unix_timestamp();
            // use time::macros::datetime;
            // read_int(r, size).map(|d| datetime!(2001-01-01 00:00:00 UTC) + time::Duration::nanoseconds(d))
            write_el_int(w, id, &unix_ts)
        }
    }

    pub mod async_ {
        use super::*;
        use time::OffsetDateTime;
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        define_read_element_id_size!(impl_async);
        define_read_vint!(impl_async);
        define_read_element_id!(impl_async);
        define_read_element_size!(impl_async);

        define_read_uint!(impl_async);
        define_read_int!(impl_async);
        define_read_float!(impl_async);

        define_read_date!(impl_async);

        pub async fn read_string<R: tokio::io::AsyncRead + Send + Unpin>(r: &mut R, size: u64) -> Result<String, MatroskaError> {
            /*FIXME - limit this to ASCII set*/
            let bytes = read_bin(r, size).await?;
            Ok(String::from_utf8(bytes).map_err(MatroskaError::UTF8)?)
        }
        pub async fn read_utf8<R: tokio::io::AsyncRead + Send + Unpin>(r: &mut R, size: u64) -> Result<String, MatroskaError> {
            let bytes = read_bin(r, size).await?;
            Ok(String::from_utf8(bytes).map_err(MatroskaError::UTF8)?)
        }
        pub async fn read_bin<R: tokio::io::AsyncRead + Send + Unpin>(r: &mut R, size: u64) -> Result<Vec<u8>, MatroskaError> {
            let mut buf = vec![0; size as usize];
            r.read_exact(&mut buf).await.map_err(MatroskaError::Io)?;
            Ok(buf)
        }

        pub async fn write_element_id_size<W: tokio::io::AsyncWrite + Send + Unpin>(w: &mut W, id: u64, size: u64) -> Result<usize, MatroskaError> {
            let buf = gen_element_id_size(id, size);
            w.write_all(&buf).await?;
            Ok(buf.len())
        }
        pub async fn write_el_uint<W: tokio::io::AsyncWrite + Send + Unpin>(w: &mut W, id: u64, val: &u64) -> Result<usize, MatroskaError> {
            let buf = gen_uint(*val);
            let header_buf = gen_element_id_size(id, buf.len() as u64);
            w.write_all(&header_buf).await?;
            w.write_all(&buf).await?;
            Ok(header_buf.len() + buf.len())
        }
        pub async fn write_el_int<W: tokio::io::AsyncWrite + Send + Unpin>(w: &mut W, id: u64, val: &i64) -> Result<usize, MatroskaError> {
            let buf = gen_int(*val);
            let header_buf = gen_element_id_size(id, buf.len() as u64);
            w.write_all(&header_buf).await?;
            w.write_all(&buf).await?;
            Ok(header_buf.len() + buf.len())
        }
        pub async fn write_el_float32<W: tokio::io::AsyncWrite + Send + Unpin>(w: &mut W, id: u64, val: &f32) -> Result<usize, MatroskaError> {
            let mut buf = vec![];
            buf.write_f32(*val).await?;
            let header_buf = gen_element_id_size(id, buf.len() as u64);
            w.write_all(&header_buf).await?;
            w.write_all(&buf).await?;
            Ok(header_buf.len() + buf.len())
        }
        pub async fn write_el_float64<W: tokio::io::AsyncWrite + Send + Unpin>(w: &mut W, id: u64, val: &f64) -> Result<usize, MatroskaError> {
            let mut buf = vec![];
            buf.write_f64(*val).await?;
            let header_buf = gen_element_id_size(id, buf.len() as u64);
            w.write_all(&header_buf).await?;
            w.write_all(&buf).await?;
            Ok(header_buf.len() + buf.len())
        }
        pub async fn write_el_string<W: tokio::io::AsyncWrite + Send + Unpin>(w: &mut W, id: u64, str: &str) -> Result<usize, MatroskaError> {
            write_el_bin(w, id, str.as_bytes()).await
        }
        pub async fn write_el_utf8<W: tokio::io::AsyncWrite + Send + Unpin>(w: &mut W, id: u64, str: &str) -> Result<usize, MatroskaError> {
            write_el_bin(w, id, str.as_bytes()).await
        }
        pub async fn write_el_bin<W: tokio::io::AsyncWrite + Send + Unpin>(w: &mut W, id: u64, buf: &[u8]) -> Result<usize, MatroskaError> {
            let header_buf = gen_element_id_size(id, buf.len() as u64);
            w.write_all(&header_buf).await?;
            w.write_all(buf).await?;
            Ok(header_buf.len() + buf.len())
        }
        pub async fn write_el_date<W: tokio::io::AsyncWrite + Send + Unpin>(w: &mut W, id: u64, val: &OffsetDateTime) -> Result<usize, MatroskaError> {
            let unix_ts = val.unix_timestamp();
            // use time::macros::datetime;
            // read_int(r, size).map(|d| datetime!(2001-01-01 00:00:00 UTC) + time::Duration::nanoseconds(d))
            write_el_int(w, id, &unix_ts).await
        }
    }


    pub fn gen_vint(val: u64) -> Result<Vec<u8>, MatroskaError> {
        let (num_bytes, bit) = match val {
            v if v <= VINT_MAX_FOR_1_BYTES => (0, 0b_1000_0000),
            v if v <= VINT_MAX_FOR_2_BYTES => (1, 0b_0100_0000),
            v if v <= VINT_MAX_FOR_3_BYTES => (2, 0b_0010_0000),
            v if v <= VINT_MAX_FOR_4_BYTES => (3, 0b_0001_0000),
            v if v <= VINT_MAX_FOR_5_BYTES => (4, 0b_0000_1000),
            v if v <= VINT_MAX_FOR_6_BYTES => (5, 0b_0000_0100),
            v if v <= VINT_MAX_FOR_7_BYTES => (6, 0b_0000_0010),
            v if v <= VINT_MAX_FOR_8_BYTES => (7, 0b_0000_0001),
            v => return Err(MatroskaError::InvalidVarIntMoreThanVintMax(v))
        };
        let mut r = vec![];
        r.push( bit | ((val >> (num_bytes)*8) as u8) );
        for i in 1..num_bytes+1 {
            r.push ( ((val >> ((num_bytes -i)*8)) & 0xFF) as u8 );
        }
        Ok(r)
    }

    pub fn gen_element_id_size(id: u64, size: u64) -> Vec<u8> {
        let mut buf = vec![];
        buf.append(&mut gen_ebml_number(id, true));
        buf.append(&mut gen_ebml_number(size, false));
        buf
    }
    pub fn gen_ebml_number(x : u64, identifier : bool) -> Vec<u8> {
        match identifier {
            true => gen_uint(x),
            false => {
                let mut r = vec!();
                if x == 0xFFFFFFFFFFFFFFFF { return vec!(0xFF); }

                let (numbytes, bit) = match x {
                    v if v < 0x100             - 0x81             => (1, 0x80),
                    v if v < 0x10000           - 0xC001           => (2, 0x40),
                    v if v < 0x1000000         - 0xE00001         => (3, 0x20),
                    v if v < 0x100000000       - 0xF0000001       => (4, 0x10),
                    v if v < 0x10000000000     - 0xF800000001     => (5, 0x08),
                    v if v < 0x1000000000000   - 0xFC0000000001   => (6, 0x04),
                    v if v < 0x100000000000000 - 0xFE000000000001 => (7, 0x02),
                    _                                             => (8, 0x01),
                };

                r.push( bit | ((x >> (numbytes-1)*8) as u8) );

                for i in 1..numbytes {
                    r.push ( ((x >> ((numbytes-i-1)*8)) & 0xFF) as u8 );
                }
                r
            },
        }
    }
    pub fn gen_uint(x : u64) -> Vec<u8> {
        let mut r = vec!();
        let numbytes = match x {
            v if v < 0x100              => 1,
            v if v < 0x10000            => 2,
            v if v < 0x1000000          => 3,
            v if v < 0x100000000        => 4,
            v if v < 0x10000000000      => 5,
            v if v < 0x1000000000000    => 6,
            v if v < 0x100000000000000  => 7,
            _                           => 8,
        };
        for i in 0..numbytes {
            r.push ( ((x >> ((numbytes-i-1)*8)) & 0xFF) as u8 );
        }
        r
    }
    pub fn gen_int(x : i64) -> Vec<u8> {
        let mut r = vec!();
        let numbytes = match x {
            v if -0x80             <= v && v < 0x80              => 1,
            v if -0x8000           <= v && v < 0x8000            => 2,
            v if -0x800000         <= v && v < 0x800000          => 3,
            v if -0x80000000       <= v && v < 0x80000000        => 4,
            v if -0x8000000000     <= v && v < 0x8000000000      => 5,
            v if -0x800000000000   <= v && v < 0x800000000000    => 6,
            v if -0x80000000000000 <= v && v < 0x80000000000000  => 7,
            _                                                    => 8,
        };
        for i in 0..numbytes {
            r.push ( ((x >> ((numbytes-i-1)*8)) & 0xFF) as u8 );
        }
        r
    }

}


#[cfg(test)]
#[allow(non_upper_case_globals)]
mod tests {
    use std::path::Path;
    use anyhow::Context;
    use rand::Rng;

    use super::*;

    fn to_binary(data: &[u8]) -> String {
        let mut strs = vec![];
        for b in data { strs.push(format!("0b_{:08b}", b)); }
        format!("[{}]", strs.join(","))
    }

    struct VintTest {
        r_size: u64,
        r: Vec<u8>,
        val: Result<u64, MatroskaError>,
        w: Vec<u8>
    }
    impl std::fmt::Debug for VintTest {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("VintTest")
                .field("size", &self.r_size)
                .field("r", &to_binary(&self.r))
                .field("val", &self.val)
                .field("w", &to_binary(&self.w))
                .finish()
        }
    }

    fn init_log() {
        let env = env_logger::Env::default()
            .filter_or("MY_LOG_LEVEL", "debug")
            .write_style_or("MY_LOG_STYLE", "always");
        env_logger::init_from_env(env);
    }

    #[test]
    fn test_vint() -> Result<(), anyhow::Error> {
        init_log();

        {
            let res = io::gen_vint(u64::MAX); assert!(res.is_err());
            let res = io::gen_vint(VINT_MAX + 1); assert!(res.is_err());
            let res = io::gen_vint(VINT_MAX); assert!(res.is_ok()); assert_eq!(res.unwrap().len(), 8);
            for i in 1..=8 {
                let vint_max = 2u64.pow((8-i) + 8*(i-1)) - 1;
                // debug!("vint_max of {i} bytes: {vint_max}");

                let res = io::gen_vint(vint_max); assert!(res.is_ok());
                assert_eq!(res.unwrap().len(), i as usize);

                let res = io::gen_vint(vint_max + 1);
                if i < 8 {
                    assert!(res.is_ok());
                    assert_eq!(res.unwrap().len(), i as usize + 1);
                } else {
                    assert!(res.is_err());
                }

                for _ in 0..10000 {
                    let val = rand::thread_rng().gen_range(0 .. vint_max);
                    let res = io::gen_vint(val); assert!(res.is_ok());
                    let data = res.unwrap();
                    let res = blocking::read_vint(&mut std::io::Cursor::new(&data)); assert!(res.is_ok());
                    let (varint, mut len) = res.unwrap();
                    assert_eq!(varint, val);
                    assert_eq!(len, data.len() as u64);
                }
            }
        }

        for _ in 0..10000 {
            let val = rand::thread_rng().gen_range(0 .. VINT_MAX);
            let res = io::gen_vint(val); assert!(res.is_ok());
            let data = res.unwrap();
            let res = blocking::read_vint(&mut std::io::Cursor::new(&data)); assert!(res.is_ok());
            let (varint, mut len) = res.unwrap();
            assert_eq!(varint, val);
            assert_eq!(len, data.len() as u64);
        }

        let mut tests = vec![];
        tests.push(VintTest {
            r: vec![0b_1111_1111], r_size: 1,
            val: Ok(0b__111_1111),
            w: vec![0b_1111_1111],
        });
        tests.push(VintTest {
            r: vec![0b_1111_1111, 0b_1111_1111], r_size: 1,
            val: Ok(0b__111_1111),
            w: vec![0b_1111_1111],
        });
        tests.push(VintTest { r_size: 1,
            r: vec![0b_1000_0000],
            val: Ok(0b_________0),
            w: vec![0b_1000_0000],
        });
        tests.push(VintTest { r_size: 1,
            r: vec![0b_0000_0000],
            val: Err(MatroskaError::InvalidUint),
            w: vec![],
        });
        tests.push(VintTest { r_size: 2,
            r: vec![0b_0100_0000, 0b_0000_0000],
            val: Ok(0b_______________________0),
            w: vec![0b_1000_0000],
        });
        tests.push(VintTest { r_size: 2,
            r: vec![0b_0111_1111, 0b_1111_1111],
            val: Ok(0b___11_1111_____1111_1111),
            w: vec![0b_0111_1111, 0b_1111_1111],
        });
        tests.push(VintTest { r_size: 8,
            r: vec![0b_0000_0001, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000],
            val: Ok(0b___________________________________________________________________________________________________________0),
            w: vec![0b_1000_0000],
        });
        tests.push(VintTest { r_size: 8,
            r: vec![0b_0000_0001, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0010],
            val: Ok(0b__________________________________________________________________________________________________________10),
            w: vec![0b_1000_0010],
        });
        tests.push(VintTest { r_size: 8,
            r: vec![0b_0000_0001, 0b_0000_1000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0010],
            val: Ok(0b____________________1000_____0000_0000_____0000_0000_____0000_0000_____0000_0000_____0000_0000_____0000_0010),
            w: vec![0b_0000_0001, 0b_0000_1000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0010],
        });
        tests.push(VintTest { r_size: 8,
            r: vec![0b_0000_0001, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111],
            val: Ok(VINT_MAX),
            w: vec![0b_0000_0001, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111],
        });
        tests.push(VintTest { r_size: 8,
            r: vec![0b_0000_0001, 0b_0000_0000, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111],
            val: Ok(0b_____________________________1111_1111_____1111_1111_____1111_1111_____1111_1111_____1111_1111_____1111_1111),
            w: vec![0b_0000_0010, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111],
        });
        for case in tests {
            let res = blocking::read_vint(&mut std::io::Cursor::new(&case.r));
            assert_eq!(case.val.is_ok(), res.is_ok(), "case: {:?}", case);
            match res {
                Ok((val, size)) => {
                    let case_val = *case.val.as_ref().unwrap();
                    assert_eq!(case_val, val, "case: {:?}", case);
                    assert_eq!(case.r_size, size, "case: {:?}", case);
                    let data = io::gen_vint(val).context(format!("failed to gen_vint val: {val}"))?;
                    assert_eq!(case.w, data, "case: {:?}", case);

                    let (val, size) = blocking::read_vint(&mut std::io::Cursor::new(&data)).context(format!("failed to read_vint val: {}, data: {:02X?}", val, case.r))?;
                    assert_eq!(case_val, val, "case: {:?}", case);
                }
                Err(err) => {
                    debug!("err: {err:?} {case:?}");
                    // assert_eq!(case.val.unwrap_err(), e, "case: {:?}", case);
                }
            }
        }

        // let mut vint: Vec<(Vec<u8>, u64)> = vec![];
        // vint.push((vec![0b_1111_1111],
        //            0b_____________111_1111));
        // vint.push((vec![0b_1000_0000],
        //            0b____________________0));
        // vint.push((vec![0b_0111_1111, 0b_1111_1111],
        //            0b______________11_1111_____1111_1111));
        // vint.push((vec![0b_0000_0001, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1111, 0b_1111_1101],
        //            0b__________________________1111_1111_____1111_1111_____1111_1111_____1111_1111_____1111_1111_____1111_1111_____1111_1101));
        // vint.push((vec![0b_0000_0001, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0000, 0b_0000_0010],
        //            0b_____________________________________________________________________________________________________________________10));
        // vint.push((vec![0xFE], 0x80 - 2));
        // vint.push((vec![0x40, 0x80], 0x80));
        // vint.push((vec![0x7F, 0xFF], 0x4000 - 1));
        // vint.push((vec![0x20, 0x40, 0x00], 0x4000));
        // vint.push((vec![0x3F, 0xFF, 0xFF], 0x200000 - 1));
        // vint.push((vec![0x10, 0x20, 0x00, 0x00], 0x200000));
        // vint.push((vec![0x1F, 0xFF, 0xFF, 0xFF], 0x10000000 - 1));
        // vint.push((vec![0x08, 0x10, 0x00, 0x00, 0x00], 0x10000000));
        // vint.push((vec![0x0F, 0xFF, 0xFF, 0xFF, 0xFF], 0x800000000 - 1));
        // vint.push((vec![0x04, 0x08, 0x00, 0x00, 0x00, 0x00], 0x800000000));
        // vint.push((vec![0x07, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], 0x40000000000 - 1));
        // vint.push((vec![0x02, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00], 0x40000000000));
        // vint.push((vec![0x03, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], 0x2000000000000 - 1));
        // for (data, val) in vint {
        //     // let mut buf = vec![];
        //     // let mut w = std::io::Cursor::new(&mut buf);
        //     // let size = write_ebml_number(&mut w, val, true).context(format!("failed to write element id {element_id} from test case '{msg}'"))?;
        //     // assert_eq!(buf, data);
        //
        //     let mut r = std::io::Cursor::new(&data);
        //     let (vint, size) = read_vint(&mut r).context(format!("failed to read_vint val: {val}, data: {data:02X?}"))?;
        //     debug!("vint: {vint}, size: {size}");
        //     assert_eq!(val, vint);
        // }



        // https://github.com/at-wat/ebml-go/blob/master/value_test.go
        let mut data_size: std::collections::BTreeMap<&'static str, (Vec<u8>, u64)> = std::collections::BTreeMap::new();
        data_size.insert("1 byte (upper bound)",  (vec![0xFE], 0x80 - 2));
        data_size.insert("2 bytes (lower bound)", (vec![0x40, 0x7F], 0x80 - 1));
        data_size.insert("2 bytes (upper bound)", (vec![0x7F, 0xFE], 0x4000 - 2));
        data_size.insert("3 bytes (lower bound)", (vec![0x20, 0x3F, 0xFF], 0x4000 - 1));
        data_size.insert("3 bytes (upper bound)", (vec![0x3F, 0xFF, 0xFE], 0x200000 - 2));
        data_size.insert("4 bytes (lower bound)", (vec![0x10, 0x1F, 0xFF, 0xFF], 0x200000 - 1));
        data_size.insert("4 bytes (upper bound)", (vec![0x1F, 0xFF, 0xFF, 0xFE], 0x10000000 - 2));
        data_size.insert("5 bytes (lower bound)", (vec![0x08, 0x0F, 0xFF, 0xFF, 0xFF], 0x10000000 - 1));
        data_size.insert("5 bytes (upper bound)", (vec![0x0F, 0xFF, 0xFF, 0xFF, 0xFE], 0x800000000 - 2));
        data_size.insert("6 bytes (lower bound)", (vec![0x04, 0x07, 0xFF, 0xFF, 0xFF, 0xFF], 0x800000000 - 1));
        data_size.insert("6 bytes (upper bound)", (vec![0x07, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE], 0x40000000000 - 2));
        data_size.insert("7 bytes (lower bound)", (vec![0x02, 0x03, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], 0x40000000000 - 1));
        data_size.insert("7 bytes (upper bound)", (vec![0x03, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE], 0x2000000000000 - 2));
        data_size.insert("8 bytes (lower bound)", (vec![0x01, 0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], 0x2000000000000 - 1));
        data_size.insert("8 bytes (upper bound)", (vec![0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE], 0xffffffffffffff - 1));
        data_size.insert("Indefinite",            (vec![0x01, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], SIZE_UNKNOWN));
        // for (msg, (data, size)) in data_size {
        //     {
        //         let mut r = std::io::Cursor::new(&data);
        //         let (val, len) = read_ebml_number(&mut r, false).context(format!("failed to read data_size from test case '{msg}', data: {data:02X?}"))?;
        //         assert_eq!(val, size);
        //     }
        //     {
        //         let mut buf = vec![];
        //         let mut w = std::io::Cursor::new(&mut buf);
        //         let len = write_ebml_number(&mut w, size, false).context(format!("failed to write data_size {size} from test case '{msg}'"))?;
        //         assert_eq!(data.len(), len);
        //         assert_eq!(buf, data);
        //     }
        // }

        let mut data_size_unknown: std::collections::BTreeMap<&'static str, Vec<u8>> = std::collections::BTreeMap::new();
        data_size_unknown.insert("1 byte",  vec![0xFF]);
        data_size_unknown.insert("2 bytes", vec![0x7F, 0xFF]);
        data_size_unknown.insert("3 bytes", vec![0x3F, 0xFF, 0xFF]);
        data_size_unknown.insert("4 bytes", vec![0x1F, 0xFF, 0xFF, 0xFF]);
        data_size_unknown.insert("5 bytes", vec![0x0F, 0xFF, 0xFF, 0xFF, 0xFF]);
        data_size_unknown.insert("6 bytes", vec![0x07, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        data_size_unknown.insert("7 bytes", vec![0x03, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        data_size_unknown.insert("8 bytes", vec![0x01, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);


        let mut vint: std::collections::BTreeMap<&'static str, (Vec<u8>, i64)> = std::collections::BTreeMap::new();
        vint.insert("1 byte (lower bound)", (vec![0x80], -0x3F));
        vint.insert("1 byte (upper bound)", (vec![0xFE], 0x3F));
        vint.insert("2 bytes (lower bound)", (vec![0x40, 0x00], -0x1FFF));
        vint.insert("2 bytes (upper bound)", (vec![0x7F, 0xFE], 0x1FFF));
        vint.insert("3 bytes (lower bound)", (vec![0x20, 0x00, 0x00], -0xFFFFF));
        vint.insert("3 bytes (upper bound)", (vec![0x3F, 0xFF, 0xFE], 0xFFFFF));
        vint.insert("4 bytes (lower bound)", (vec![0x10, 0x00, 0x00, 0x00], -0x7FFFFFF));
        vint.insert("4 bytes (upper bound)", (vec![0x1F, 0xFF, 0xFF, 0xFE], 0x7FFFFFF));
        vint.insert("5 bytes (lower bound)", (vec![0x08, 0x00, 0x00, 0x00, 0x00], -0x3FFFFFFFF));
        vint.insert("5 bytes (upper bound)", (vec![0x0F, 0xFF, 0xFF, 0xFF, 0xFE], 0x3FFFFFFFF));
        vint.insert("6 bytes (lower bound)", (vec![0x04, 0x00, 0x00, 0x00, 0x00, 0x00], -0x1FFFFFFFFFF));
        vint.insert("6 bytes (upper bound)", (vec![0x07, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE], 0x1FFFFFFFFFF));
        vint.insert("7 bytes (lower bound)", (vec![0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], -0xFFFFFFFFFFFF));
        vint.insert("7 bytes (upper bound)", (vec![0x03, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE], 0xFFFFFFFFFFFF));
        vint.insert("8 bytes (lower bound)", (vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], -0x7FFFFFFFFFFFFF));
        vint.insert("8 bytes (upper bound)", (vec![0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE], 0x7FFFFFFFFFFFFF));

        Ok(())
    }



    #[test]
    fn parse_ebml_number_test_1() {
        {
            let identifier = true;
            let input = io::gen_ebml_number(0x1A45DFA3, identifier);
            assert_eq!(input.len(), 4);
            assert_eq!(input, [0x1A,0x45,0xDF,0xA3]);
            let (val, size) = blocking::read_element_id(&mut &input[..]).unwrap();
            assert_eq!(val, 0x1A45DFA3);
            assert_eq!(size, 4);
        }
        {
            let identifier = false;
            let input = io::gen_ebml_number(0x0A45DFA3, identifier);
            assert_eq!(input, [0x1A,0x45,0xDF,0xA3]);
            let (val, size) = blocking::read_element_size(&mut &input[..]).unwrap();
            assert_eq!(val.unwrap(), 0x0A45DFA3);
            assert_eq!(size, 4);
        }
        {
            let identifier = false;
            let input = io::gen_ebml_number(0x0A45, identifier);
            assert_eq!(input, [0x4A,0x45]);
            let (val, size) = blocking::read_element_size(&mut &input[..]).unwrap();
            assert_eq!(val.unwrap(), 0x0A45);
            assert_eq!(size, 2);

            let input = [0x4A,0x45,0xDF,0xA3];
            let (val, size) = blocking::read_element_size(&mut &input[..]).unwrap();
            assert_eq!(val.unwrap(), 0x0A45);
            assert_eq!(size, 2);
        }
        {
            let input = [0xFF,0x7F];
            let res = blocking::read_element_size(&mut &input[..]);
            assert_eq!(res.is_err(), false);
        }
        {
            let input = [0x7F,0xFF];
            let res = blocking::read_element_size(&mut &input[..]);
            assert_eq!(res.is_err(), false);
        }
        {
            let input = [0x00];
            let res = blocking::read_element_size(&mut &input[..]);
            assert_eq!(res.is_err(), true);
        }
        {
            let input = [0x40];
            let res = blocking::read_element_size(&mut &input[..]);
            assert_eq!(res.is_err(), true);
        }
        {
            let input = [];
            let res = blocking::read_element_id(&mut &input[..]);
            assert_eq!(res.is_err(), true);
        }
    }
}
