use tokio::fs::{File, remove_file};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let filename = "foo.txt";
    let mut f = File::create(filename).await?;
    let n = f.write(b"some bytes").await?;
    f.write_all(b"some bytes").await?;
    println!("Wrote the first {} bytes of 'some bytes'.", n);

    let mut reader: &[u8] = b"hello";
    io::copy(&mut reader, &mut f).await?;

    let mut f = File::open(filename).await?;
    let mut buffer = [0;10];
    let n = f.read(&mut buffer[..]).await?;
    println!("The {} bytes: {:?}", n, &buffer[..n]);

    let mut f = File::open(filename).await?;
    let mut buffer = Vec::new();
    let a = f.read_to_end(&mut buffer).await?;
    println!("The {} bytes: {:?}", a, &buffer[..]);
    remove_file(filename).await?;

    Ok(())
}
