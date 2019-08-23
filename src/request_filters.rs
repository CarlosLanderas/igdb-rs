use crate::request_builder::{Equality, OrderBy, RequestBuilder};

const ALL_FIELDS: &'static str = "*";

#[derive(Clone)]
pub(crate) struct Filter {
    pub(crate) key: String,
    pub(crate) symbol: String,
    pub(crate) value: String,
}

impl RequestBuilder {
    pub fn all_fields(&mut self) -> &mut Self {
        self.fields.clear();
        self.fields.push(ALL_FIELDS.into());
        self
    }

    pub fn add_field<S: Into<String>>(&mut self, field: S) -> &mut Self {
        self.fields.push(field.into());
        self
    }

    pub fn add_fields<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let str_fields: Vec<String> = iter.into_iter().map(Into::into).collect();
        self.fields.extend(str_fields);
        self
    }

    pub fn add_where_in(&mut self, field: String, values: Vec<String>) -> &mut Self {
        self.filters.push(Filter {
            key: field,
            symbol: String::new(),
            value: format!("= ({})", values.join(",")),
        });

        self
    }

    pub fn add_where<L: Into<String>, R: Into<String>>(
        &mut self,
        field: L,
        equality: Equality,
        clause: R,
    ) -> &mut Self {
        self.filters.push(Filter {
            key: field.into(),
            symbol: equality.to_string(),
            value: clause.into(),
        });
        self
    }

    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = limit;
        self
    }

    pub fn contains<S: Into<String>>(&mut self, field: S, value: S) -> &mut Self {
        self.filters.push(Filter {
            key: field.into(),
            symbol: String::new(),
            value: format!("~ *\"{}\"*", value.into()),
        });
        self
    }

    pub fn search<S: Into<String>>(&mut self, search: S) -> &mut Self {
        self.search = search.into();
        self
    }

    pub fn sort_by<S: Into<String>>(&mut self, field: S, order: OrderBy) -> &mut Self {
        self.sort = (field.into(), order.to_string());
        self
    }
}
