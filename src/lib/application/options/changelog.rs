use crate::{
    application::error::changelog_options_invariant_error::ChangelogOptionsInvariantError,
    usecase::type_aliases::AnyError,
};

pub const FORMAT_PLACEHOLDER: &str = "%s";

#[derive(Debug)]
pub struct ChangelogOptions {
    generate_from_latest_version: bool,
    format: ChangelogFormatOptions,
    exclude_trigger: Option<String>,
}

impl ChangelogOptions {
    pub fn new(
        generate_from_latest_version: bool,
        format: ChangelogFormatOptions,
        exclude_trigger: Option<String>,
    ) -> Self {
        ChangelogOptions {
            generate_from_latest_version,
            format,
            exclude_trigger,
        }
    }

    pub fn generate_from_latest_version(&self) -> bool {
        self.generate_from_latest_version
    }

    pub fn format(&self) -> &ChangelogFormatOptions {
        &self.format
    }

    pub fn exclude_trigger(&self) -> &Option<String> {
        &self.exclude_trigger
    }
}

#[derive(Debug)]
pub struct ChangelogFormatOptions {
    title_format: String,
    type_format: String,
    scope_format: String,
    list_format: String,
    item_format: String,
    breaking_format: String,
}

impl ChangelogFormatOptions {
    pub fn new(
        title_format: String,
        type_format: String,
        scope_format: String,
        list_format: String,
        item_format: String,
        breaking_format: String,
    ) -> Result<Self, AnyError> {
        Self::ensure_format_has_placeholder(&title_format, "title")?;
        Self::ensure_format_has_placeholder(&type_format, "type")?;
        Self::ensure_format_has_placeholder(&scope_format, "scope")?;
        Self::ensure_format_has_placeholder(&list_format, "list")?;
        Self::ensure_format_has_placeholder(&item_format, "item")?;
        Self::ensure_format_has_placeholder(&breaking_format, "breaking")?;
        Ok(ChangelogFormatOptions {
            title_format,
            type_format,
            scope_format,
            list_format,
            item_format,
            breaking_format,
        })
    }

    pub fn title(&self) -> &str {
        &self.title_format
    }
    pub fn typ(&self) -> &str {
        &self.type_format
    }
    pub fn scope(&self) -> &str {
        &self.scope_format
    }
    pub fn list(&self) -> &str {
        &self.list_format
    }
    pub fn item(&self) -> &str {
        &self.item_format
    }
    pub fn breaking(&self) -> &str {
        &self.breaking_format
    }

    fn ensure_format_has_placeholder(
        format_string: &str,
        format_target: &str,
    ) -> Result<(), ChangelogOptionsInvariantError> {
        if !format_string.contains(FORMAT_PLACEHOLDER) {
            return Err(ChangelogOptionsInvariantError::new(&format!(
                "{} format must contain placeholder",
                format_target
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::application::options::changelog::ChangelogFormatOptions;

    #[test]
    fn ensure_format_has_placeholder_correct() {
        let format_string = "= %s";
        let result = ChangelogFormatOptions::ensure_format_has_placeholder(
            &format_string.to_owned(),
            "test",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn ensure_format_has_placeholder_wrong() {
        let format_string = "not a correct format string";
        let result = ChangelogFormatOptions::ensure_format_has_placeholder(
            &format_string.to_owned(),
            "test",
        );
        assert!(result.is_err());
    }
}
