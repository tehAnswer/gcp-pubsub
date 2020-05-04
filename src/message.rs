use crate::presenters::EncodedMessage;
use crate::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    #[serde(alias = "ackId")]
    pub(crate) ack_id: String,
    pub message: EncodedMessage,
}

impl Message {
    pub fn payload<T: DeserializeOwned>(&self) -> Result<T, Error> {
        let decoded_message = self.message.decode();
        decoded_message
            .map_err(Error::Base64)
            .and_then(|json_bytes| serde_json::from_slice(&json_bytes).map_err(Error::Json))
    }
}

mod tests {
    use crate::presenters::EncodedMessage;
    use crate::{Error, Message};
    use serde::{de::DeserializeOwned, Deserialize, Serialize};

    #[test]

    fn message_payload_test() {
        #[derive(Deserialize, Serialize, Default, Debug)]
        struct Foo {
            pub bar: i64,
        }

        let data = Foo { bar: 10000 };
        let encoded_message = EncodedMessage::new(&data);
        let message = Message {
            ack_id: "1".into(),
            message: encoded_message,
        };

        let payload_result: Result<Foo, Error> = message.payload();
        assert_eq!(payload_result.is_err(), false);
        let payload = payload_result.unwrap();
        assert_eq!(payload.bar, data.bar);
    }
}
