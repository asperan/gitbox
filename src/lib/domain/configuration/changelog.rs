use crate::domain::trigger::Trigger;

pub struct ChangelogConfiguration<'a> {
    generate_from_latest_version: bool,
    format: ChangelogFormat<'a>,
    exclude_trigger: Option<Trigger>,
}

impl<'a> ChangelogConfiguration<'a> {
    pub fn new(
        from_latest_version: bool,
        format: ChangelogFormat,
        exclude_trigger: Option<Trigger>,
    ) -> ChangelogConfiguration {
        ChangelogConfiguration {
            generate_from_latest_version: from_latest_version,
            format,
            exclude_trigger,
        }
    }

    pub fn generate_from_latest_version(&self) -> bool {
        self.generate_from_latest_version
    }

    pub fn format(&self) -> &ChangelogFormat {
        &self.format
    }

    pub fn exclude_trigger(&self) -> &Option<Trigger> {
        &self.exclude_trigger
    }
}

pub type ChangelogTransformer<'a> = Box<dyn Fn(&String) -> String + 'a>;

pub struct ChangelogFormat<'a> {
    title: ChangelogTransformer<'a>,
    typ: ChangelogTransformer<'a>,
    scope: ChangelogTransformer<'a>,
    list: ChangelogTransformer<'a>,
    item: ChangelogTransformer<'a>,
    breaking: ChangelogTransformer<'a>,
}

impl<'a> ChangelogFormat<'a> {
    pub fn new(
        title: ChangelogTransformer<'a>,
        typ: ChangelogTransformer<'a>,
        scope: ChangelogTransformer<'a>,
        list: ChangelogTransformer<'a>,
        item: ChangelogTransformer<'a>,
        breaking: ChangelogTransformer<'a>,
    ) -> ChangelogFormat<'a> {
        ChangelogFormat {
            title,
            typ,
            scope,
            list,
            item,
            breaking,
        }
    }

    pub fn title(&self) -> &ChangelogTransformer {
        &self.title
    }

    pub fn typ(&self) -> &ChangelogTransformer {
        &self.typ
    }

    pub fn scope(&self) -> &ChangelogTransformer {
        &self.scope
    }

    pub fn list(&self) -> &ChangelogTransformer {
        &self.list
    }

    pub fn item(&self) -> &ChangelogTransformer {
        &self.item
    }

    pub fn breaking(&self) -> &ChangelogTransformer {
        &self.breaking
    }
}
