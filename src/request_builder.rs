use crate::request_filters::Filter;
use std::future::Future;
use std::str::FromStr;
use std::string::ToString;
use url::Url;

const HEADER_CLIENT_ID: &str = "Client-ID";
const HEADER_AUTH: &str = "Authorization";

#[derive(Clone)]
/// Request Builder struct
pub struct RequestBuilder {
    pub(crate) fields: Vec<String>,
    pub(crate) filters: Vec<Filter>,
    pub(crate) sort: (String, String),
    pub(crate) limit: usize,
    pub(crate) search: String,
}

impl Default for RequestBuilder {
    fn default() -> Self {
        RequestBuilder {
            fields: Vec::new(),
            filters: vec![],
            sort: (String::new(), String::new()),
            limit: 10,
            search: String::new(),
        }
    }
}

pub enum OrderBy {
    Descending,
    Ascending,
}

impl ToString for OrderBy {
    fn to_string(&self) -> String {
        match self {
            OrderBy::Ascending => "asc",
            OrderBy::Descending => "desc",
        }
        .into()
    }
}

pub enum Equality {
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
}

impl ToString for Equality {
    fn to_string(&self) -> String {
        match self {
            Equality::Equal => "=",
            Equality::NotEqual => "!=",
            Equality::Greater => ">",
            Equality::Less => "<",
            Equality::GreaterOrEqual => ">=",
            Equality::LessOrEqual => "<=",
        }
        .into()
    }
}

impl RequestBuilder {
    /// Creates a new Request Builder
    ///It can also be created using IGDBClient::create_request();
    /// # Examples
    /// ```
    ///use igdb_rs::request_builder::RequestBuilder;
    ///
    /// let mut request = RequestBuilder::new();
    /// ```
    /// ```
    ///use igdb_rs::client::IGDBClient;
    ///
    ///let mut request = IGDBClient::create_request();
    /// ```
    pub fn new() -> RequestBuilder {
        RequestBuilder::default()
    }

    pub(crate) fn build(
        &self,
        client_id: &str,
        token: &str,
        url: &str,
    ) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>> {
        let body = self.build_body();

        log::debug!("url: {}, body: {}", url, body);
        let client = reqwest::Client::new();
        let req = client
            .post(Url::from_str(url).unwrap())
            .body(body)
            .header(HEADER_CLIENT_ID, client_id)
            .header(HEADER_AUTH, format!("Bearer {}", token))
            .header("content-type", "application/json")
            .send();
        req
    }

    pub(crate) fn build_body(&self) -> String {
        let fields = self
            .fields
            .iter()
            .enumerate()
            .fold(String::new(), |mut acc, (i, v)| {
                acc.push_str(v);
                let separator = match i {
                    _ if i == (self.fields.len() - 1) => ";",
                    _ => ",",
                };
                acc.push_str(separator);
                acc
            });

        let filter_clause =
            self.filters
                .iter()
                .enumerate()
                .fold(String::new(), |mut acc, (i, filter)| {
                    if i == 0 {
                        acc.push_str("where ")
                    }
                    if i != 0 {
                        acc.push_str(" & ")
                    };

                    acc.push_str(&format!(
                        "{} {} {}",
                        filter.key, filter.symbol, filter.value
                    ));

                    if i == (self.filters.len() - 1) {
                        acc.push_str(";");
                    }
                    acc
                });

        self.format_body_parts(fields, filter_clause)
    }

    fn format_body_parts(&self, fields: String, filters: String) -> String {
        let mut order = String::new();

        let mut body = format!("fields {}", fields);

        if !str::is_empty(&self.search) {
            body = format!("{} search \"{}\";", body, self.search);
        }

        if !self.filters.is_empty() {
            body = format!("{} {}", body, filters);
        }

        if !str::is_empty(&self.sort.0) {
            order.push_str(&format!("sort {} {}", self.sort.0, self.sort.1));
            body = format!("{} {};", body, order);
        }

        body = format!("{} limit {};", body, self.limit);
        body
    }
}

#[test]
fn request_builder_with_all_fields() {
    let mut builder = RequestBuilder::new();

    builder.all_fields();

    let body = builder.build_body();

    assert_eq!("fields *; limit 10;", &body);
}

#[test]
fn request_builder_with_fields_and_where_clause_body_build() {
    let mut builder = RequestBuilder::new();

    builder
        .add_field("name")
        .add_field("involved_companies")
        .add_where("name", Equality::Equal, "Conan")
        .add_where("id", Equality::Less, 39047.to_string());

    let body = builder.build_body();

    assert_eq!(
        "fields name,involved_companies; where name = Conan & id < 39047; limit 10;",
        &body
    );
}

#[test]
fn request_builder_with_fields_where_clause_and_sort_asc_body_build() {
    let mut builder = RequestBuilder::new();

    builder
        .add_field("name")
        .add_field("involved_companies")
        .add_where("id", Equality::Equal, 39047.to_string())
        .add_where("name", Equality::Equal, "Conan")
        .sort_by("name", OrderBy::Ascending)
        .limit(2);

    let body = builder.build_body();

    assert_eq!(
        "fields name,involved_companies; where id = 39047 & name = Conan; sort name asc; limit 2;",
        &body
    );
}
