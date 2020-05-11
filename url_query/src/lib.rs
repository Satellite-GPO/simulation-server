use std::{
    collections::BTreeMap,
    ops::Index,
    str::FromStr
};

pub struct UrlQuery(BTreeMap<String, Option<String>>);

impl UrlQuery {
    pub fn from(query_str: &str) -> Self {
        Self(query_str.split('&')
            .filter(|x| x != &"")
            .map(|expr| {
                let mut values = expr.split('=')
                    .filter(|x| x != &"");

                (String::from(values.next().unwrap()),
                 match values.next() {
                     Some(s) => Some(String::from(s)),
                     None => None,
                 })
            })
            .collect())
    }

    /// Returns option of option with following semantics: if the outer option
    /// is None then there is no such key in query. If the inner option is
    /// None then the key presents in the query without any value:
    /// ...&key&...
    pub fn get(&self, key: &str) -> Option<Option<String>> {
        match self.0.get(key) {
            Some(x) => Some(x.as_ref().cloned()),
            None => None,
        }
    }

    pub fn get_of_type<'a, T>(&self, name: &'a str) 
        -> Result<T, &'a str>
        where T: FromStr 
    {
        match self.get(name).as_ref() {
            Some(value) => {
                match value {
                    Some(raw) => {
                        match raw.parse() {
                            Ok(value) => Ok(value),
                            Err(_) => Err("Error parsing value"),
                        }
                    },
                    None => Err("No value mapped to given key")
                }
            },
            None => {
                Err("No key in query")
            },
        }
    }
}


// TODO: rewrite in a correct manner
impl Index<&str> for UrlQuery {
    type Output = Option<String>;

    fn index(&self, key: &str) -> &Self::Output {
        &self.0[key]
    }
}

#[cfg(test)]
mod tests {
    use crate::UrlQuery;

    #[test]
    fn from() {
        let string = "action=get_users&sort=by_name&order=increasing&cached=&";
        let q = UrlQuery::from(string);
        assert_eq!(q["action"], Some(String::from("get_users")));
        assert_eq!(q["sort"], Some(String::from("by_name")));
        assert_eq!(q["order"], Some(String::from("increasing")));
        assert_eq!(q["cached"], None);
    }
}
