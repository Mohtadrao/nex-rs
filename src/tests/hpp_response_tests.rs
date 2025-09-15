
#[cfg(test)]
mod hpp_response_tests {
    use crate::hpp::dispatcher::SimpleHandler;
    use bytes::Bytes;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_handler_returns_response() {
        let handler = Arc::new(SimpleHandler::new());
        handler.register(7, |b: Bytes| async move {
            // echo the body back with prefix
            let mut v = vec![1u8];
            v.extend_from_slice(&b);
            Ok(Bytes::from(v))
        });
        let mut body = vec![];
        body.extend_from_slice(&7u32.to_le_bytes());
        body.extend_from_slice(&[9,9,9]);
        let payload = Bytes::from(body);
        let framed = crate::hpp::packet::HppPacket::encode(&payload);
        let res = handler.on_hpp_message(framed.into()).await.expect(\"handler call\").expect(\"some response\");
        assert_eq!(res[0], 1u8);
        assert_eq!(res.len(), 4);
    }
}
