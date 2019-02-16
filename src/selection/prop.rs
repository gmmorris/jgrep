use json::JsonValue;
use regex::Regex;

pub fn prop(prop_value: String) -> Box<Fn(Option<&JsonValue>) -> Option<&JsonValue>> {
  Box::new(move |input: Option<&JsonValue>| match input {
    Some(json) => match json {
      JsonValue::Object(ref object) => object.get(&prop_value),
      _ => None,
    },
    None => None,
  })
}

pub fn greedily_matches(maybe_pattern: Option<&str>) -> Result<Option<&str>, Option<&str>> {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^\.(?P<prop>[[:word:]]+)$").unwrap();
  }

  fn match_prop(pattern: &str) -> Option<&str> {
    RE.captures(pattern)
      .and_then(|cap| cap.name("prop").map(|prop| prop.as_str()))
  }

  match maybe_pattern {
    Some(pattern) => match match_prop(pattern) {
      Some(prop_value) => Ok(None),
      None => Err(maybe_pattern),
    },
    None => Err(maybe_pattern),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use json::object;

  #[test]
  fn should_match_prop() {
    assert_eq!(greedily_matches(Some(".id")), Ok(None));
  }

  #[test]
  fn shouldnt_match_identity() {
    assert_eq!(greedily_matches(Some(".")), Err(Some(".")));
  }

  #[test]
  fn should_return_none_when_json_isnt_present() {
    assert_eq!(prop(String::from(".id"))(None), None);
  }

  #[test]
  fn should_return_json_prop_when_json_has_prop() {
    let ref data = object! {
        "name"    => "John Doe",
        "age"     => 30
    };

    assert_eq!(prop(String::from("name"))(Some(data)), Some(&data["name"]));
  }
}
