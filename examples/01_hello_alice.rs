use schemas::person_capnp::person;

fn main() {
    // A capnp message is a tree of structs. The root struct is the message itself.
    // The message is created with a builder, which is a mutable reference to the message.
    // The type of the message is specified with the generic parameter, in this case
    // person::Owned. This is a struct generated by the capnp compiler.
    let mut message = capnp::message::TypedBuilder::<person::Owned>::new_default();

    // The root struct is accessed with the `init_root` method. It returns a mutable
    // reference to the root struct.
    let mut person = message.init_root();

    // The fields of the struct are accessed with methods generated by the capnp compiler.
    person.set_name("Alice".into());
    person.set_email("alice@bob.com".into());

    let mut f = std::fs::File::create("alice.bin").unwrap();

    // The message is serialized with the `write_message` function. It takes a mutable
    // reference to a writer and a reference to the message, `&message.into_inner()` when
    // using a TypedBuilder.
    capnp::serialize::write_message(&mut f, &message.into_inner()).unwrap();

    println!("Wrote alice.bin");
    println!("Try running: \n\tcapnp decode schemas/person.capnp Person < alice.bin");
}
