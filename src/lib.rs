use crate::cfg::{AnnotationMode, Config};
use std::fmt::Debug;

pub mod cfg;

pub trait Expander<T> {
    /// Expands a value, applying annotations as configured.
    ///
    /// # Arguments
    ///
    /// * `value`: The value to expand.
    ///
    /// returns: T
    fn expand(&self, value: T) -> T;
}

#[derive(Debug, Default)]
pub struct JsonExpander {
    config: Config,
}

impl JsonExpander {
    /// Creates a new JsonExpander with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config`: The configuration to use.
    ///
    /// returns: JsonExpander
    ///
    /// # Examples
    ///
    /// ```
    /// let config = jxpand::cfg::Config::default();
    /// let expander = jxpand::JsonExpander::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        JsonExpander {
            config: config.resolve(),
        }
    }

    /// Gets the configuration used by the expander.
    pub fn config(&self) -> &Config {
        &self.config
    }
}

impl Expander<serde_json::Value> for JsonExpander {
    fn expand(&self, value: serde_json::Value) -> serde_json::Value {
        use serde_json::Value;
        let annotations = self.config.annotations();
        if annotations.none() {
            return value;
        }

        let object_mode = self.config.object_mode();

        match value {
            Value::Object(map) => {
                let mut new_map = serde_json::Map::new();
                for (k, v) in map {
                    new_map.insert(k, self.expand(v));
                }
                Value::Object(new_map)
            }
            Value::Array(values) => {
                let count = values.len();
                let mut new_values = Vec::new();
                for (i, v) in values.into_iter().enumerate() {
                    let mut wrapper = match v {
                        Value::Object(map) => match object_mode {
                            AnnotationMode::Wrap => {
                                let mut wrapper = serde_json::Map::new();
                                wrapper.insert("value".to_string(), Value::Object(map));
                                wrapper
                            }
                            AnnotationMode::Merge => {
                                let mut new_map = serde_json::Map::new();
                                for (k, v) in map {
                                    new_map.insert(k, self.expand(v));
                                }
                                new_map
                            }
                        },
                        _ => {
                            let mut wrapper = serde_json::Map::new();
                            wrapper.insert("value".to_string(), self.expand(v));
                            wrapper
                        }
                    };

                    if annotations.index().is_enabled() {
                        wrapper.insert(annotations.index().annotation(), Value::Number(i.into()));
                    }
                    if annotations.first().is_enabled() {
                        wrapper.insert(annotations.first().annotation(), Value::Bool(i == 0));
                    }
                    if annotations.last().is_enabled() {
                        wrapper
                            .insert(annotations.last().annotation(), Value::Bool(i == count - 1));
                    }

                    new_values.push(Value::Object(wrapper));
                }
                if annotations.count().is_enabled() {
                    let mut wrapper = serde_json::Map::new();
                    wrapper.insert("values".to_string(), Value::Array(new_values));
                    if annotations.count().is_enabled() {
                        wrapper.insert(
                            annotations.count().annotation(),
                            Value::Number(count.into()),
                        );
                    }
                    Value::Object(wrapper)
                } else {
                    Value::Array(new_values)
                }
            }
            _ => value,
        }
    }
}

/// Expands a JSON value using the default configuration.
///
/// # Arguments
///
/// * `value`: The value to expand.
///
/// returns: Value
///
/// # Examples
///
/// ```
/// let value = serde_json::json!([1, 2, 3]);
/// let result = jxpand::expand_json(value);
/// ```
pub fn expand_json(value: serde_json::Value) -> serde_json::Value {
    let expander = JsonExpander::default();
    expander.expand(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_expand_array_integers() {
        let value = json!([1, 2, 3]);
        let result = expand_json(value);
        assert_eq!(
            result,
            json!({
                "values": [
                    {"index": 0, "first": true, "last": false, "value": 1},
                    {"index": 1, "first": false, "last": false, "value": 2},
                    {"index": 2, "first": false, "last": true, "value": 3},
                ],
                "count": 3,
            })
        );
    }

    #[test]
    fn test_expand_array_objects_wrapped_by_default() {
        let value = json!([
            {"name": "Alice", "age": 42},
            {"name": "Bob", "age": 43},
            {"name": "Carol", "age": 44},
        ]);
        let result = expand_json(value);
        assert_eq!(
            result,
            json!({
                "values": [
                    {"index": 0, "first": true, "last": false, "value": {"name": "Alice", "age": 42}},
                    {"index": 1, "first": false, "last": false, "value": {"name": "Bob", "age": 43}},
                    {"index": 2, "first": false, "last": true, "value": {"name": "Carol", "age": 44}},
                ],
                "count": 3,
            })
        );
    }
}
