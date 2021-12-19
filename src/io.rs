use std::{
    path::Path,
    io::{
        Result as IoResult,
        SeekFrom
    }
};

use tokio::{
    fs::File,
    io::{
        BufReader,
        AsyncReadExt,
        AsyncSeekExt
    }
};

pub struct FileReader {
    reader: BufReader<File>
}

impl FileReader {

    #[inline]
    pub async fn new<P: AsRef<Path>>(path: P) -> IoResult<Self> {
        Result::Ok(Self {
            reader: BufReader::new(File::open(path).await?)
        })
    }

    #[inline]
    pub async fn read<const N: usize>(&mut self) -> IoResult<[u8; N]> {
        let mut buffer = [0; N];
        self.reader.read_exact(&mut buffer).await?;
        return Result::Ok(buffer);
    }

    #[inline]
    pub async fn seek(&mut self, n: i64) -> IoResult<()> {
        self.reader.seek(SeekFrom::Current(n)).await?;
        return Result::Ok(());
    }

}
