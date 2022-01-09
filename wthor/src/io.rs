use std::{
    path::Path,
    io::SeekFrom
};

use anyhow::Result;

use tokio::{
    fs::File,
    io::{
        BufReader,
        AsyncReadExt,
        AsyncSeekExt
    }
};

#[derive(Debug)]
pub struct FileReader {
    reader: BufReader<File>
}

impl FileReader {

    #[inline]
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        Result::Ok(Self {
            reader: BufReader::new(File::open(path).await?)
        })
    }

    #[inline]
    pub async fn read<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut buffer = [0; N];
        self.reader.read_exact(&mut buffer).await?;
        return Result::Ok(buffer);
    }

    #[inline]
    pub async fn seek(&mut self, n: i64) -> Result<()> {
        self.reader.seek(SeekFrom::Current(n)).await?;
        return Result::Ok(());
    }

}
