use async_std::io::{Read, Write};
use async_std::net::TcpStream;
#[cfg(feature = "secure")]
use async_tls::client::TlsStream;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Data Stream used for communications
#[pin_project::pin_project(project = DataStreamProj)]
pub enum DataStream {
    Tcp(#[pin] TcpStream),
    #[cfg(feature = "secure")]
    Ssl(#[pin] TlsStream<TcpStream>),
}

impl DataStream {
    /// Unwrap the stream into TcpStream. This method is only used in secure connection.
    pub fn into_tcp_stream(self) -> Option<TcpStream> {
        match self {
            DataStream::Tcp(stream) => Some(stream),
            #[cfg(feature = "secure")]
            DataStream::Ssl(_) => None,
        }
    }

    /// Test if the stream is secured
    pub fn is_ssl(&self) -> bool {
        match self {
            #[cfg(feature = "secure")]
            DataStream::Ssl(_) => true,
            _ => false,
        }
    }

    /// Returns a reference to the underlying TcpStream.
    pub fn get_ref(&self) -> &TcpStream {
        match self {
            DataStream::Tcp(ref stream) => stream,
            #[cfg(feature = "secure")]
            DataStream::Ssl(ref stream) => stream.get_ref(),
        }
    }
}

impl Read for DataStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        match self.project() {
            DataStreamProj::Tcp(stream) => stream.poll_read(cx, buf),
            #[cfg(feature = "secure")]
            DataStreamProj::Ssl(stream) => stream.poll_read(cx, buf),
        }
    }
}

impl Write for DataStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match self.project() {
            DataStreamProj::Tcp(stream) => stream.poll_write(cx, buf),
            #[cfg(feature = "secure")]
            DataStreamProj::Ssl(stream) => stream.poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.project() {
            DataStreamProj::Tcp(stream) => stream.poll_flush(cx),
            #[cfg(feature = "secure")]
            DataStreamProj::Ssl(stream) => stream.poll_flush(cx),
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.project() {
            DataStreamProj::Tcp(stream) => stream.poll_close(cx),
            #[cfg(feature = "secure")]
            DataStreamProj::Ssl(stream) => stream.poll_close(cx),
        }
    }
}
