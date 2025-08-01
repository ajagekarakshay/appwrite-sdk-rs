//! File input handling for uploads

use crate::error::{AppwriteError, Result};
use bytes::Bytes;
use std::path::Path;
use tokio::fs;

/// Represents a file input for upload operations
#[derive(Debug, Clone)]
pub struct InputFile {
    /// The source type of the file
    pub source_type: InputFileSource,
    /// The filename
    pub filename: String,
    /// The MIME type
    pub mime_type: Option<String>,
    /// File data for bytes source
    pub data: Option<Bytes>,
    /// File path for path source
    pub path: Option<String>,
}

/// Source type for input files
#[derive(Debug, Clone)]
pub enum InputFileSource {
    /// File from filesystem path
    Path,
    /// File from byte data
    Bytes,
}

impl InputFile {
    /// Create an InputFile from a filesystem path
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| AppwriteError::file_error("Invalid filename"))?
            .to_string();

        let mime_type = mime_guess::from_path(path)
            .first()
            .map(|mime| mime.to_string());

        let path_str = path
            .to_str()
            .ok_or_else(|| AppwriteError::file_error("Invalid path encoding"))?
            .to_string();

        Ok(Self {
            source_type: InputFileSource::Path,
            filename,
            mime_type,
            data: None,
            path: Some(path_str),
        })
    }

    /// Create an InputFile from byte data
    pub fn from_bytes<T: Into<Bytes>>(
        data: T,
        filename: impl Into<String>,
        mime_type: Option<String>,
    ) -> Self {
        Self {
            source_type: InputFileSource::Bytes,
            filename: filename.into(),
            mime_type,
            data: Some(data.into()),
            path: None,
        }
    }

    /// Get the file size
    pub async fn size(&self) -> Result<u64> {
        match &self.source_type {
            InputFileSource::Path => {
                let path = self.path.as_ref()
                    .ok_or_else(|| AppwriteError::file_error("Path not set"))?;
                let metadata = fs::metadata(path).await?;
                Ok(metadata.len())
            }
            InputFileSource::Bytes => {
                let data = self.data.as_ref()
                    .ok_or_else(|| AppwriteError::file_error("Data not set"))?;
                Ok(data.len() as u64)
            }
        }
    }

    /// Read the file data
    pub async fn read_data(&self) -> Result<Bytes> {
        match &self.source_type {
            InputFileSource::Path => {
                let path = self.path.as_ref()
                    .ok_or_else(|| AppwriteError::file_error("Path not set"))?;
                let data = fs::read(path).await?;
                Ok(Bytes::from(data))
            }
            InputFileSource::Bytes => {
                self.data.as_ref()
                    .ok_or_else(|| AppwriteError::file_error("Data not set"))
                    .map(|data| data.clone())
            }
        }
    }

    /// Read a chunk of the file data
    pub async fn read_chunk(&self, offset: u64, size: usize) -> Result<Bytes> {
        match &self.source_type {
            InputFileSource::Path => {
                let path = self.path.as_ref()
                    .ok_or_else(|| AppwriteError::file_error("Path not set"))?;
                
                use tokio::io::{AsyncReadExt, AsyncSeekExt};
                let mut file = fs::File::open(path).await?;
                file.seek(std::io::SeekFrom::Start(offset)).await?;
                
                let mut buffer = vec![0u8; size];
                let bytes_read = file.read(&mut buffer).await?;
                buffer.truncate(bytes_read);
                
                Ok(Bytes::from(buffer))
            }
            InputFileSource::Bytes => {
                let data = self.data.as_ref()
                    .ok_or_else(|| AppwriteError::file_error("Data not set"))?;
                
                let start = offset as usize;
                let end = std::cmp::min(start + size, data.len());
                
                if start >= data.len() {
                    Ok(Bytes::new())
                } else {
                    Ok(data.slice(start..end))
                }
            }
        }
    }

    /// Get the filename
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Get the MIME type
    pub fn mime_type(&self) -> Option<&str> {
        self.mime_type.as_deref()
    }

    /// Set the filename
    pub fn set_filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = filename.into();
        self
    }

    /// Set the MIME type
    pub fn set_mime_type(mut self, mime_type: Option<String>) -> Self {
        self.mime_type = mime_type;
        self
    }
}