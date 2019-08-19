use crate::v3::endpoints::games;
use std::borrow::BorrowMut;
use std::collections::BTreeMap;
use std::str::{FromStr};
use std::string::{ToString};
use surf::middleware::HttpClient;
use url::Url;

const ALL_FIELDS: &'static str = "*";

pub struct RequestBuilder {
    pub(crate) fields: Vec<String>,
    pub(crate) where_clauses: BTreeMap<String, (Equality, String)>,
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
            OrderBy::Descending => "desc"
        }.into()
    }
}

pub enum Equality {
    Lower,
    Greater,
    Equal
}

impl ToString for Equality {
    fn to_string(&self) -> String {
        match self {
            Equality::Equal => "=",
            Equality::Greater => ">",
            Equality::Lower => "<"
        }.into()
    }
}


impl RequestBuilder {
    pub fn new() -> RequestBuilder {
        RequestBuilder {
            fields: Vec::new(),
            where_clauses: BTreeMap::new(),
            sort: (String::new(), String::new()),
        }
    }
    pub fn all_fields(&mut self) -> &Self {
        self.fields.clear();
        self.fields.push(ALL_FIELDS.into());
        self
    }

    pub fn add_field<S: Into<String>>(&mut self, field: S) -> &mut Self {
        self.fields.push(field.into());
        self
    }

    pub fn add_where<L: Into<String>, R: Into<String>>(
        &mut self,
        field: L,
        equality: Equality,
        clause: R,
    ) -> &mut Self {
        self.where_clauses.insert(field.into(), (equality, clause.into()));
        self
    }

    pub fn sort_by<S: Into<String>>(&mut self, field: S, order: OrderBy) {
        self.sort = (field.into(), order.to_string());
    }

    fn build(&self) -> surf::Request<impl HttpClient> {
        let mut req = surf::Request::new(http::Method::GET, Url::from_str(&games()).unwrap());
        let mut_req = req.borrow_mut();
        req
    }

    pub(crate) fn build_body(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

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

    fn format_body_parts(&self, fields: String, where_clauses : String) -> String {

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
    let mut builder = RequestBuilder::new();

    builder.all_fields();

    let body = builder.build_body();

    assert_eq!(
        "fields *;",
        String::from_utf8_lossy(&body).to_owned()
    );
}

#[test]
fn request_builder_with_fields_and_where_clause_body_build() {
    let mut builder = RequestBuilder::new();

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
    let mut builder = RequestBuilder::new();

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