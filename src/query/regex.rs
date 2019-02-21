use serde::{Deserialize, Serialize};
use tantivy::query::{Query, RegexQuery as TantivyRegexQuery};
use tantivy::schema::Schema;

use crate::query::{CreateQuery, KeyValue};
use crate::{Error, Result};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct RegexQuery {
    regex: KeyValue<String>,
}

impl CreateQuery for RegexQuery {
    fn create_query(self, schema: &Schema) -> Result<Box<Query>> {
        let KeyValue { field, value } = self.regex;
        let field = schema
            .get_field(&field)
            .ok_or_else(|| Error::QueryError(format!("Field: {} does not exist", field)))?;
        Ok(Box::new(TantivyRegexQuery::new(value, field)))
    }
}
