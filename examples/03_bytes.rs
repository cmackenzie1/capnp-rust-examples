use bytes::{Buf, BufMut, BytesMut};
use schemas::person_capnp::person;

fn main() {
    // Another way to serialize the message is to use a bytes::BufMut as the writer.
    let mut buffer = BytesMut::new().writer();

    let people = vec![("Alice", "alice@bob.com"), ("Jim", "jim@bob.com")];
    for person in people {
        let mut message = capnp::message::TypedBuilder::<person::Owned>::new_default();
        let mut p = message.init_root();
        p.set_name(person.0.into());
        p.set_email(person.1.into());

        capnp::serialize::write_message(&mut buffer, &message.into_inner()).unwrap();
    }

    // The BufMut can be converted to a Buf and read back with a reader.
    // Be sure to convert the BufMut to a Buf before reading.
    let mut reader = buffer.into_inner().reader();
    while let Ok(Some(message_reader)) = capnp::serialize::try_read_message(
        &mut reader, // &mut buffer.as_slice() is wrong!
        capnp::message::ReaderOptions::new(),
    ) {
        let person = message_reader.get_root::<person::Reader>().unwrap();
        println!(
            "{} <{}>",
            person.get_name().unwrap().to_str().unwrap(),
            person.get_email().unwrap().to_str().unwrap(),
        );
    } // loop terminates when the buffer is empty and `read_message` returns Ok(None)
}
