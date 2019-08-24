use crate::request_builder::{Equality, OrderBy, RequestBuilder};

const ALL_FIELDS: &'static str = "*";

#[derive(Clone)]
///Filter struct
/// It's values represents any kind of filter, like:
/// id >= 5
/// name = "Conan"
pub(crate) struct Filter {
    pub(crate) key: String,
    pub(crate) symbol: String,
    pub(crate) value: String,
}
///This struct allows creating a custom and parameterized request to IGDB endpoints by using it's filter methods
impl RequestBuilder {
    ///Requests all field for the given query
    /// # Examples
    /// ```
    /// use igdb_rs::client::IGDBClient;
    ///
    /// let mut request = IGDBClient::create_request();
    /// request
    /// .all_fields()
    /// .search("Conan")
    /// .limit(1);
    /// ```
    pub fn all_fields(&mut self) -> &mut Self {
        self.fields.clear();
        self.fields.push(ALL_FIELDS.into());
        self
    }
    /// Adds one field to be retrieved for this request
    ///Requests all field for the given query
    /// # Examples
    /// ```
    /// use igdb_rs::client::IGDBClient;
    ///
    /// let mut request = IGDBClient::create_request();
    /// request
    /// .add_field("description")
    /// .add_field("name")
    /// .search("Borderlands");
    /// ```
    pub fn add_field<S: Into<String>>(&mut self, field: S) -> &mut Self {
        self.fields.push(field.into());
        self
    }

    /// Adds several fields for this request by using an Iterator object
    /// # Examples
    /// ```
    /// use igdb_rs::client::IGDBClient;
    ///
    /// let mut request = IGDBClient::create_request();
    /// request
    /// .add_fields(vec!["description", "name", "summary"])
    /// .contains("name", "Mass Effect");
    /// ```
    pub fn add_fields<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let str_fields: Vec<String> = iter.into_iter().map(Into::into).collect();
        self.fields.extend(str_fields);
        self
    }

    /// Adds a filter representing a group of elements like WHERE IN statement in SQL
    /// # Examples
    /// ```
    /// use igdb_rs::client::IGDBClient;
    ///
    /// let mut request = IGDBClient::create_request();
    /// request
    /// .all_fields()
    /// .add_where_in("name".to_owned(), vec!["5".to_owned(), "8".to_owned(), "10".to_owned()])
    /// .limit(5);
    /// ```
    pub fn add_where_in(&mut self, field: String, values: Vec<String>) -> &mut Self {
        self.filters.push(Filter {
            key: field,
            symbol: String::new(),
            value: format!("= ({})", values.join(",")),
        });

        self
    }

    /// Adds a filter under using given condition
    /// # Examples
    /// ```
    /// use igdb_rs::client::IGDBClient;
    /// use igdb_rs::request_builder::Equality;
    ///
    /// let mut request = IGDBClient::create_request();
    /// request
    /// .add_where("id", Equality::GreaterOrEqual, "20")
    /// .add_where("onlinemax", Equality::LessOrEqual, "12")
    /// .contains("name", "Fighter");
    /// ```
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

    /// Limits the registries obtained from the server
    /// # Examples
    /// ```
    /// use igdb_rs::client::IGDBClient;
    /// use igdb_rs::request_builder::Equality;
    ///
    /// let mut request = IGDBClient::create_request();
    /// request
    /// .all_fields()
    /// .limit(8);
    /// ```
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = limit;
        self
    }

    /// Return the registries containing the given *value* for the specified field.
    /// Can start and end with anything
    /// # Examples
    /// ```
    /// use igdb_rs::client::IGDBClient;
    /// use igdb_rs::request_builder::Equality;
    ///
    /// let mut request = IGDBClient::create_request();
    /// request
    /// .all_fields()
    /// .limit(8);
    /// ```
    pub fn contains<S: Into<String>>(&mut self, field: S, value: S) -> &mut Self {
        self.filters.push(Filter {
            key: field.into(),
            symbol: String::new(),
            value: format!("~ *\"{}\"*", value.into()),
        });
        self
    }

    /// Search based on name, results are sorted by similarity to the given search string.
    /// Searchable endpoints: - Characters - Collections - Games - People - Platforms - Themes
    /// # Examples
    /// ```
    /// use igdb_rs::client::IGDBClient;
    /// use igdb_rs::request_builder::Equality;
    ///
    /// let mut request = IGDBClient::create_request();
    /// request
    /// .search("Zelda")
    /// .limit(8);
    /// ```
    pub fn search<S: Into<String>>(&mut self, search: S) -> &mut Self {
        self.search = search.into();
        self
    }

    /// Sorts the query by the given field
    /// # Examples
    /// ```
    /// use igdb_rs::client::IGDBClient;
    /// use igdb_rs::request_builder::{Equality, OrderBy};
    ///
    /// let mut request = IGDBClient::create_request();
    /// request
    /// .add_field("name")
    /// .sort_by("name", OrderBy::Descending)
    /// .limit(8);
    /// ```
    pub fn sort_by<S: Into<String>>(&mut self, field: S, order: OrderBy) -> &mut Self {
        self.sort = (field.into(), order.to_string());
        self
    }
}
