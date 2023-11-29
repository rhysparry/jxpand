use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Annotation {
    annotation: String,
    enabled: bool,
}

impl Annotation {
    /// Creates a new annotation that is enabled by default.
    ///
    /// # Arguments
    ///
    /// * `annotation`: The name of the annotation.
    ///
    /// returns: Annotation
    ///
    /// # Examples
    ///
    /// ```
    /// let annotation = jxpand::cfg::Annotation::enabled("foo".to_string());
    /// assert_eq!(annotation.annotation(), "foo");
    /// assert_eq!(annotation.is_enabled(), true);
    /// ```
    pub fn enabled(annotation: String) -> Self {
        Annotation {
            annotation,
            enabled: true,
        }
    }

    /// Creates a new annotation that is disabled by default.
    ///
    /// # Arguments
    ///
    /// * `annotation`: The name of the annotation.
    ///
    /// returns: Annotation
    ///
    /// # Examples
    ///
    /// ```
    /// let annotation = jxpand::cfg::Annotation::disabled("foo".to_string());
    /// assert_eq!(annotation.annotation(), "foo");
    /// assert_eq!(annotation.is_enabled(), false);
    /// ```
    pub fn disabled(annotation: String) -> Self {
        Annotation {
            annotation,
            enabled: false,
        }
    }

    /// Creates a new annotation.
    ///
    /// # Arguments
    ///
    /// * `annotation`: The name of the annotation.
    /// * `enabled`: Whether the annotation is enabled.
    ///
    /// returns: Annotation
    ///
    /// # Examples
    ///
    /// ```
    /// let annotation = jxpand::cfg::Annotation::new("foo".to_string(), true);
    /// assert_eq!(annotation.annotation(), "foo");
    /// assert_eq!(annotation.is_enabled(), true);
    /// ```
    ///
    /// ```
    /// let annotation = jxpand::cfg::Annotation::new("foo".to_string(), false);
    /// assert_eq!(annotation.annotation(), "foo");
    /// assert_eq!(annotation.is_enabled(), false);
    /// ```
    pub fn new(annotation: String, enabled: bool) -> Self {
        Annotation {
            annotation,
            enabled,
        }
    }

    /// Returns the name of the annotation.
    pub fn annotation(&self) -> String {
        self.annotation.clone()
    }

    /// Returns whether the annotation is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Returns a new annotation with the given prefix.
    ///
    /// # Arguments
    ///
    /// * `prefix`: The prefix to apply to the annotation
    ///
    /// returns: Annotation
    ///
    /// # Examples
    ///
    /// ```
    /// let annotation = jxpand::cfg::Annotation::enabled("foo".to_string());
    /// let prefixed = annotation.prefix("bar_");
    /// assert_eq!(prefixed.annotation(), "bar_foo");
    /// ```
    pub fn prefix(&self, prefix: &str) -> Annotation {
        Annotation::new(format!("{}{}", prefix, self.annotation), self.enabled)
    }
}

impl Display for Annotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let enabled = if self.enabled { "enabled" } else { "disabled" };
        write!(f, "{}({})", self.annotation, enabled)
    }
}

#[derive(Debug)]
pub struct Annotations {
    count: Annotation,
    first: Annotation,
    last: Annotation,
    index: Annotation,
}

impl Annotations {
    /// Gets the configuration for the count annotation.
    pub fn count(&self) -> &Annotation {
        &self.count
    }
    /// Gets the configuration for the first annotation.
    pub fn first(&self) -> &Annotation {
        &self.first
    }
    /// Gets the configuration for the last annotation.
    pub fn last(&self) -> &Annotation {
        &self.last
    }
    /// Gets the configuration for the index annotation.
    pub fn index(&self) -> &Annotation {
        &self.index
    }

    /// Returns whether all annotations are disabled.
    pub fn none(&self) -> bool {
        !self.count.is_enabled()
            && !self.first.is_enabled()
            && !self.last.is_enabled()
            && !self.index.is_enabled()
    }

    /// Applies a prefix to all annotations.
    ///
    /// # Arguments
    ///
    /// * `prefix`: A string to prefix all annotations with.
    ///
    /// returns: Annotations
    ///
    /// # Examples
    ///
    /// ```
    /// let annotations = jxpand::cfg::Annotations::default();
    /// let prefixed = annotations.prefix("foo_");
    /// assert_eq!(prefixed.count().to_string(), "foo_count(enabled)");
    /// assert_eq!(prefixed.first().to_string(), "foo_first(enabled)");
    /// assert_eq!(prefixed.last().to_string(), "foo_last(enabled)");
    /// assert_eq!(prefixed.index().to_string(), "foo_index(enabled)");
    /// ```
    pub fn prefix(&self, prefix: &str) -> Annotations {
        Annotations {
            count: self.count.prefix(prefix),
            first: self.first.prefix(prefix),
            last: self.last.prefix(prefix),
            index: self.index.prefix(prefix),
        }
    }

    /// Disables all annotations.
    pub fn disable(&mut self) {
        self.count.enabled = false;
        self.first.enabled = false;
        self.last.enabled = false;
        self.index.enabled = false;
    }

    /// Enables all annotations.
    pub fn enable(&mut self) {
        self.count.enabled = true;
        self.first.enabled = true;
        self.last.enabled = true;
        self.index.enabled = true;
    }

    /// Disables the count annotation.
    pub fn disable_count(&mut self) {
        self.count.enabled = false;
    }

    /// Enables the count annotation.
    pub fn enable_count(&mut self) {
        self.count.enabled = true;
    }

    /// Disables the first annotation.
    pub fn disable_first(&mut self) {
        self.first.enabled = false;
    }

    /// Enables the first annotation.
    pub fn enable_first(&mut self) {
        self.first.enabled = true;
    }

    /// Disables the last annotation.
    pub fn disable_last(&mut self) {
        self.last.enabled = false;
    }

    /// Enables the last annotation.
    pub fn enable_last(&mut self) {
        self.last.enabled = true;
    }

    /// Disables the index annotation.
    pub fn disable_index(&mut self) {
        self.index.enabled = false;
    }

    /// Enables the index annotation.
    pub fn enable_index(&mut self) {
        self.index.enabled = true;
    }

    /// Sets the name of the count annotation.
    pub fn set_count_annotation(&mut self, name: &str) {
        self.count.annotation = name.to_string();
    }

    /// Sets the name of the first annotation.
    pub fn set_first_annotation(&mut self, name: &str) {
        self.first.annotation = name.to_string();
    }

    /// Sets the name of the last annotation.
    pub fn set_last_annotation(&mut self, name: &str) {
        self.last.annotation = name.to_string();
    }

    /// Sets the name of the index annotation.
    pub fn set_index_annotation(&mut self, name: &str) {
        self.index.annotation = name.to_string();
    }
}

impl Default for Annotations {
    fn default() -> Self {
        Annotations {
            count: Annotation::enabled("count".to_string()),
            first: Annotation::enabled("first".to_string()),
            last: Annotation::enabled("last".to_string()),
            index: Annotation::enabled("index".to_string()),
        }
    }
}

/// The mode to use when annotating objects.
#[derive(Clone, Debug, Default, ValueEnum)]
pub enum AnnotationMode {
    /// Wrap the object in a wrapper object containing the annotations.
    #[default]
    Wrap,
    /// Merge the annotations into the object using a prefix.
    Merge,
}

#[derive(Debug)]
pub struct Config {
    annotations: Annotations,
    annotation_prefix: String,
    object_mode: AnnotationMode,
    resolved: bool,
}

impl Config {
    /// Creates a new configuration.
    ///
    /// # Arguments
    ///
    /// * `annotations`: The annotation configuration to use.
    /// * `annotation_prefix`: The prefix to apply to annotations.
    /// * `object_mode`: The mode to use when annotating objects.
    ///
    /// returns: Config
    pub fn new(
        annotations: Annotations,
        annotation_prefix: String,
        object_mode: AnnotationMode,
    ) -> Self {
        Config {
            annotations,
            annotation_prefix,
            object_mode,
            resolved: false,
        }
    }
    /// Gets the configuration for the annotations.
    pub fn annotations(&self) -> &Annotations {
        &self.annotations
    }
    /// Gets the prefix to use for annotations.
    pub fn annotation_prefix(&self) -> &str {
        &self.annotation_prefix
    }
    /// Gets the mode to use when annotating objects.
    pub fn object_mode(&self) -> &AnnotationMode {
        &self.object_mode
    }

    /// Returns a new configuration with the prefix applied to all annotations
    /// depending on the mode.
    pub fn resolve(self) -> Config {
        if self.resolved {
            return self;
        }

        Config {
            annotations: match self.object_mode {
                AnnotationMode::Wrap => self.annotations,
                AnnotationMode::Merge => self.annotations.prefix(&self.annotation_prefix),
            },
            annotation_prefix: self.annotation_prefix,
            object_mode: self.object_mode,
            resolved: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            annotations: Annotations::default(),
            annotation_prefix: "_".to_string(),
            object_mode: AnnotationMode::default(),
            resolved: false,
        }
    }
}
