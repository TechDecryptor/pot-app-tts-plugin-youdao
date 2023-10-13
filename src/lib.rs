use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn tts(
    text: &str,
    lang: &str,
    _needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;

    let url = "https://dict.youdao.com/dictvoice";

    let res = client
        .get(url)
        .query(&[("audio", text), ("le", lang)])
        .send()?
        .bytes()?;

    let result = res.to_vec();

    Ok(json!(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let needs = HashMap::new();
        let result = tts("你好", "zh", needs).unwrap();
        println!("{result}");
    }
}
