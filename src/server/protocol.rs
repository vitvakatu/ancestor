use std::io;

use bytes::BytesMut;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Decoder, Encoder, Framed};
use tokio_proto::pipeline::ServerProto;

use {decode_message, encode_message, Request, Response};

pub struct Codec;

impl Decoder for Codec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        decode_message(buf)
    }
}

impl Encoder for Codec {
    type Item = Response;
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        encode_message(msg, buf)
    }
}

pub struct Protocol;

impl<T: AsyncWrite + AsyncRead + 'static> ServerProto<T> for Protocol {
    type Request = Request;
    type Response = Response;

    type Transport = Framed<T, Codec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(Codec))
    }
}
