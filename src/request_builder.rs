use std::collections::BTreeMap;
use std::str::FromStr;
use std::string::ToString;
use surf::middleware::HttpClient;
use url::Url;

const ALL_FIELDS: &'static str = "*";
const HEADER_KEY_NAME: &'static str = "user-key";

pub struct RequestBuilder {
    url: String,
    api_key: String,
    fields: Vec<String>,
    where_clauses: BTreeMap<String, (Equality, String)>,
    sort: (String, String),
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
    Lower,
    Greater,
    Equal,
}

impl ToString for Equality {
    fn to_string(&self) -> String {
        match self {
            Equality::Equal => "=",
            Equality::Greater => ">",
            Equality::Lower => "<",
        }
        .into()
    }
}

impl RequestBuilder {
    pub fn new<S: Into<String>>(api_key: S, url: S) -> RequestBuilder {
        RequestBuilder {
            api_key: api_key.into(),
            url: url.into(),
            fields: Vec::new(),
            where_clauses: BTreeMap::new(),
            sort: (String::new(), String::new()),
        }
    }
    pub fn all_fields(mut self) -> Self {
        self.fields.clear();
        self.fields.push(ALL_FIELDS.into());
        self
    }

    pub fn add_field<S: Into<String>>(mut self, field: S) -> Self {
        self.fields.push(field.into());
        self
    }

    pub fn add_fields<I, T>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let str_fields: Vec<String> = iter.into_iter().map(Into::into).collect();
        self.fields.extend(str_fields);
        self
    }

    pub fn add_where<L: Into<String>, R: Into<String>>(
        mut self,
        field: L,
        equality: Equality,
        clause: R,
    ) -> Self {
        self.where_clauses
            .insert(field.into(), (equality, clause.into()));
        self
    }

    pub fn sort_by<S: Into<String>>(&mut self, field: S, order: OrderBy) {
        self.sort = (field.into(), order.to_string());
    }

    pub(crate) fn build(&self) -> surf::Request<impl HttpClient> {
        let body = &self.build_body();

        let mut req = surf::Request::new(http::Method::GET, Url::from_str(&self.url).unwrap())
            .body_bytes(body);

        req.headers().insert(HEADER_KEY_NAME, &self.api_key);
        req.headers().insert("content-type", "application/text");

        req
    }

    pub(crate) fn build_body(&self) -> Vec<u8> {
        let bytes: Vec<u8> = vec![];

        let fields = self
            .fields
            .iter()
            .enumerate()
            .fold(String::new(), |mut acc, (i, v)| {
                acc.push_str(v);
                let separator = match i {
                    v if i == (self.fields.len() - 1) => ";",
                    _ => ",",
                };
                acc.push_str(separator);
                acc
            });

        let where_clause =
            self.where_clauses
                .iter()
                .enumerate()
                .fold(String::new(), |mut acc, (i, (k, v))| {
                    if i == 0 {
                        acc.push_str("where ")
                    }
                    if i != 0 {
                        acc.push_str(" & ")
                    };

                    acc.push_str(&format!("{} {} {}", k, v.0.to_string(), v.1));

                    if i == (self.where_clauses.len() - 1) {
                        acc.push_str(";");
                    }
                    acc
                });

        self.format_body_parts(fields, where_clause)
            .as_bytes()
            .to_vec()
    }

    fn format_body_parts(&self, fields: String, where_clauses: String) -> String {
        let mut order = String::new();

        let mut body = format!("fields {}", fields);

        if self.where_clauses.len() > 0 {
            body = format!("{} {}", body, where_clauses);
        }

        if !str::is_empty(&self.sort.0) {
            order.push_str(&format!("sort {} {}", self.sort.0, self.sort.1));
            body = format!("{} {}", body, order);
        }

        body
    }
}

#[test]
fn request_builder_with_all_fields() {
    let mut builder = RequestBuilder::new("", "");

    builder.all_fields();

    let body = builder.build_body();

    assert_eq!("fields *;", String::from_utf8_lossy(&body).to_owned());
}

#[test]
fn request_builder_with_fields_and_where_clause_body_build() {
    let mut builder = RequestBuilder::new("", "");

    builder
        .add_field("name")
        .add_field("involved_companies")
        .add_where("name", Equality::Equal, "Conan")
        .add_where("id", Equality::Lower, 39047.to_string());

    let body = builder.build_body();

    assert_eq!(
        "fields name,involved_companies; where id < 39047 & name = Conan;",
        String::from_utf8_lossy(&body).to_owned()
    );
}

#[test]
fn request_builder_with_fields__where_clause_and_sort_asc_body_build() {
    let mut builder = RequestBuilder::new("", "");

    builder
        .add_field("name")
        .add_field("involved_companies")
        .add_where("name", Equality::Equal, "Conan")
        .add_where("id", Equality::Equal, 39047.to_string())
        .sort_by("name", OrderBy::Ascending);

    let body = builder.build_body();

    assert_eq!(
        "fields name,involved_companies; where id = 39047 & name = Conan; sort name asc",
        String::from_utf8_lossy(&body).to_owned()
    );
}
