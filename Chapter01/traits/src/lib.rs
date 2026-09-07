use std::io::{Read, Write};
// Structs

///
/// Configuration for our application
///
pub struct Config {
    values: Vec<(String, String)>,
}

///
/// A service for managing a configuration
///
pub struct KeyValueConfigService {}

// Traits

///
/// Provides a get() function to return values associated with
/// the specified key.
///
pub trait ValueGetter {
    fn get(&self, s: &str) -> Option<String>;
}

///
/// Write a config
///
pub trait ConfigWriter {
    fn write(&self, config: Config, to: &mut impl Write) -> std::io::Result<()>;
}

///
/// Read a config
///
pub trait ConfigReader {
    fn read(&self, from: &mut impl Read) -> std::io::Result<Config>;
}

// Impls

impl Config {
    pub fn new(values: Vec<(String, String)>) -> Config {
        Config { values: values }
    }
}

impl KeyValueConfigService {
    pub fn new() -> KeyValueConfigService {
        KeyValueConfigService {}
    }
}

impl ConfigWriter for KeyValueConfigService {
    fn write(&self, config: Config, mut to: &mut impl Write) -> std::io::Result<()> {
        for v in config.values {
            writeln!(&mut to, "{0}={1}", v.0, v.1)?;
        }
        Ok(())
    }
}

impl ConfigReader for KeyValueConfigService {
    fn read(&self, from: &mut impl Read) -> std::io::Result<Config> {
        let mut buffer = String::new();
        from.read_to_string(&mut buffer)?;

        // chain iterators together and collect the results
        let values: Vec<(String, String)> = buffer
            .split_terminator("\n") // split
            .map(|line| line.trim()) // remove whitespace
            .filter(|line| {
                // filter invalid lines
                let pos = line.find("=").unwrap_or(0);
                pos > 0 && pos < line.len() - 1
            })
            .map(|line| {
                // split only once so values can legally contain '='.
                let mut parts = line.splitn(2, "=");
                let key = parts.next().unwrap_or_default().to_string();
                let value = parts.next().unwrap_or_default().to_string();
                (key, value)
            })
            .collect(); // transform it into a vector
        Ok(Config::new(values))
    }
}

impl ValueGetter for Config {
    fn get(&self, s: &str) -> Option<String> {
        self.values.iter().find_map(|tuple| {
            if &tuple.0 == s {
                Some(tuple.1.clone())
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn config_get_value() {
        let config = Config::new(vec![("hello".to_string(), "world".to_string())]);
        assert_eq!(config.get("hello"), Some("world".to_string()));
        assert_eq!(config.get("HELLO"), None);
    }

    #[test]
    fn keyvalueconfigservice_write_config() {
        let config = Config::new(vec![("hello".to_string(), "world".to_string())]);

        let service = KeyValueConfigService::new();
        let mut target = vec![];
        assert!(service.write(config, &mut target).is_ok());

        assert_eq!(
            String::from_utf8(target).unwrap(),
            "hello=world\n".to_string()
        );
    }

    #[test]
    fn keyvalueconfigservice_read_config() {
        let service = KeyValueConfigService::new();
        let readable = &format!("{}\n{}", "hello=world", "a=b").into_bytes();

        let config = service
            .read(&mut Cursor::new(readable))
            .expect("Couldn't read from the vector");

        assert_eq!(
            config.values,
            vec![
                ("hello".to_string(), "world".to_string()),
                ("a".to_string(), "b".to_string())
            ]
        );
    }

    #[test]
    fn config_get_value_missing() {
        // Regression: missing keys should return None.
        let config = Config::new(vec![]);
        assert_eq!(config.get("missing"), None);
    }

    #[test]
    fn keyvalueconfigservice_read_ignores_invalid_lines() {
        // Regression: lines without a proper key=value pair should be ignored.
        let service = KeyValueConfigService::new();
        let input = b"hello=world\ninvalid\n=a\nb=\nkey=value";
        let config = service.read(&mut Cursor::new(input)).unwrap();
        assert_eq!(config.values.len(), 2);
        assert_eq!(config.get("hello"), Some("world".to_string()));
        assert_eq!(config.get("key"), Some("value".to_string()));
    }

    #[test]
    fn keyvalueconfigservice_read_trims_whitespace() {
        // Regression: leading and trailing whitespace around lines should be trimmed,
        // while spaces around the '=' separator are preserved as part of the key/value.
        let service = KeyValueConfigService::new();
        let input = b"  hello=world  \n\tkey=value\t";
        let config = service.read(&mut Cursor::new(input)).unwrap();
        assert_eq!(config.values.len(), 2);
        assert_eq!(config.get("hello"), Some("world".to_string()));
        assert_eq!(config.get("key"), Some("value".to_string()));
    }

    #[test]
    fn keyvalueconfigservice_round_trip() {
        // Regression: writing and reading back a config should preserve values.
        let service = KeyValueConfigService::new();
        let config = Config::new(vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "2".to_string()),
        ]);
        let mut buffer = vec![];
        service.write(config, &mut buffer).unwrap();
        let read_back = service.read(&mut Cursor::new(&buffer)).unwrap();
        assert_eq!(read_back.get("a"), Some("1".to_string()));
        assert_eq!(read_back.get("b"), Some("2".to_string()));
    }

    #[test]
    fn config_duplicate_keys_first_wins() {
        // Regression: document current behavior for duplicate keys.
        let config = Config::new(vec![
            ("key".to_string(), "first".to_string()),
            ("key".to_string(), "second".to_string()),
        ]);
        assert_eq!(config.get("key"), Some("first".to_string()));
    }

    #[test]
    fn keyvalueconfigservice_read_value_with_equals() {
        // Regression: values containing '=' should be preserved after the first separator.
        let service = KeyValueConfigService::new();
        let input = b"equation=2+2=4";
        let config = service.read(&mut Cursor::new(input)).unwrap();
        assert_eq!(config.get("equation"), Some("2+2=4".to_string()));
    }

    #[test]
    fn keyvalueconfigservice_read_empty_input() {
        // Regression: empty inputs should parse to an empty config.
        let service = KeyValueConfigService::new();
        let config = service.read(&mut Cursor::new(b"")).unwrap();
        assert_eq!(config.values.len(), 0);
    }

    #[test]
    fn keyvalueconfigservice_read_only_newlines() {
        // Regression: whitespace-only lines should not produce key-value entries.
        let service = KeyValueConfigService::new();
        let config = service.read(&mut Cursor::new(b"\n\n  \n\t\n")).unwrap();
        assert_eq!(config.values.len(), 0);
    }
}
