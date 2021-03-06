use std::{io, net, string};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("url error {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("Socks version: {0} not supported")]
    NotSupportedSocksVersion(u8),
    #[error("Version: {0} not supported")]
    NotSupportedVersion(u8),
    #[error("io error {0}")]
    Io(#[from] io::Error),
    #[error("string from utf8 error {0}")]
    Utf8Error(#[from] string::FromUtf8Error),
    #[error("Net address parse {0}")]
    StdParseAddr(#[from] net::AddrParseError),
    #[error("Unimplement feature")]
    Unimplement,
    #[error("Auth method not accepted")]
    MethodNotAccept,
    #[error("Unknown auth method: {0}")]
    MethodUnknown(u8),
    #[error("Wrong method")]
    MethodWrong,
    #[error("No get socket address")]
    SocketAddr,
    #[error("Wrong reserved byte: {0}")]
    WrongReserved(u8),
    #[error("Address type: {0} not supported")]
    AddressTypeNotSupported(u8),
    #[error("Unknown command: {0}")]
    CommandUnknown(u8),
    #[error("Parse ip version 6")]
    ParseIPv6,
    #[error("Parse host")]
    ParseHost,
    #[error("Parse port")]
    ParsePort,
    #[error("Unsupported scheme: {0}")]
    UnsupportedScheme(String),
    #[error("Empty scheme")]
    EmptyScheme,
    #[error("Empty authority")]
    EmptyAuthority,
    #[error("Username len more when 255: {0}")]
    UnameLenOverflow(usize),
    #[error("Password len more when 255: {0}")]
    PasswdLenOverflow(usize),
    #[error("Wrong status: {0}")]
    WrongStatus(u8),
    #[error("General SOCKS server failure")]
    ReplyGeneralFailure,
    #[error("Connection not allowed by ruleset")]
    ReplyConnectionNotAllowed,
    #[error("Network unreachable")]
    ReplyNetworkUnreachable,
    #[error("Host unreachable")]
    ReplyHostUnreachable,
    #[error("Connection refused")]
    ReplyConnectionRefused,
    #[error("TTL expired")]
    ReplyTtlExpired,
    #[error("Command not supported")]
    ReplyCommandNotSupported,
    #[error("Address type not supported")]
    ReplyAddressTypeNotSupported,
    #[error("Reply unassigned: {0}")]
    ReplyUnassigned(u8),
    #[error("No set username for url")]
    BadUsername,
    #[error("No set password for url")]
    BadPassword,
}
