use uuid::Uuid;

use crate::types::ExchangeId;

pub fn generate_cloid(_: ExchangeId) -> String {
    uuid_to_hex_string(Uuid::new_v4())
}

pub(crate) fn uuid_to_hex_string(uuid: Uuid) -> String {
    let hex_string = uuid
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<String>>()
        .join("");
    format!("0x{hex_string}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_gen_client_id() {
        // Implicitly calls my_vector.into_iter()
        let id = uuid_to_hex_string(Uuid::new_v4());
        println!("cloid {} | len: {}", &id, id.len());
    }

    #[test]
    fn parse_client_id() {
        // Implicitly calls my_vector.into_iter()
        let id = uuid_to_hex_string(Uuid::new_v4());
        let uuid = Uuid::try_parse(&id.strip_prefix("0x").unwrap()).unwrap();
        println!("cloid {} | len: {}", &id, id.len());
        println!("uuid {:?} ", &uuid);
    }
}
