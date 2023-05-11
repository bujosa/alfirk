// Example of a macro that receives an expression as a parameter
macro_rules! pubsub_subscribe {
    ($topic:expr) => {
        pub fn subscribe_to_topic() {
            println!("Subscribed to topic: {}", $topic);
        }
    };
}

pubsub_subscribe!("topic_1");
fn main() {
    // Aquí iría el código para conectarse al servicio de pubsub y suscribirse al tema especificado
    // println!("Subscribed to topic: {}", "topic_1");
    subscribe_to_topic();
}
