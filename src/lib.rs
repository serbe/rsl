pub mod error;
pub mod socks5;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use uri::Uri;

    #[tokio::test]
    async fn conn() {
        let mut stream = socks5::connect_uri(&"socks5://127.0.0.1:5959".parse::<Uri>().unwrap(), &"http://api.ipify.org".parse::<Uri>().unwrap()).await.unwrap();
        stream.write_all(b"GET / HTTP/1.0\r\n\r\n").await.unwrap();
        stream.flush().await.unwrap();
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).await.unwrap();
        assert!(!buf.is_empty());
    }
}