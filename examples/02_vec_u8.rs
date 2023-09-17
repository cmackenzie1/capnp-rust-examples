use std::io::BufReader;

use schemas::person_capnp::person;

fn main() {
    // A simple way to serialize the message is to use a Vec<u8> as the writer.
    // The message is serialized into the buffer and can be read back with a reader.
    // Note, when reading from the buffer, the message must be read with a reader
    // that can advance the buffer. The `std::io::BufReader` type can be used for this.
    let mut buffer = Vec::new();

    let people = vec![("Alice", "alice@bob.com"), ("Jim", "jim@bob.com")];
    for person in people {
        let mut message = capnp::message::TypedBuilder::<person::Owned>::new_default();
        let mut p = message.init_root();
        p.set_name(person.0.into());
        p.set_email(person.1.into());

        capnp::serialize::write_message(&mut buffer, &message.into_inner()).unwrap();
    }

    // The buffer can be read back with a reader. The `read_message` function
    // takes a mutable reference to the buffer and returns a message reader.
    // Be careful to pass the buffer as a mutable reference, otherwise the
    // loop will never terminate as it will keep reading the same message.
    let mut reader = BufReader::new(buffer.as_slice());
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
