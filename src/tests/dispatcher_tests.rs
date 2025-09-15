
#[cfg(test)]
mod dispatcher_tests {
    use crate::hpp::dispatcher::SimpleHandler;
    use bytes::Bytes;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_register_and_invoke() {
        let handler = Arc::new(SimpleHandler::new());
        handler.register(123, |b: Bytes| async move {
            assert_eq!(b.len(), 3);
            Ok(Bytes::from(vec![1,2,3]))
        });
        // Build an HPP-framed RMC message: method id 123 + body [9,9,9]
        let mut v = vec![];
        v.extend_from_slice(&123u32.to_le_bytes());
        v.extend_from_slice(&[9,9,9]);
        let payload = Bytes::from(v);
        // Construct HPP frame (length-prefixed)
        let framed = crate::hpp::packet::HppPacket::encode(&payload);
        // Invoke handler
        let _ = handler.on_hpp_message(framed.into()).await;
    }
}
