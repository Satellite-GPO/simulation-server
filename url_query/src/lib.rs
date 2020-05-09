use std::{
    collections::BTreeMap,
    ops::Index,
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
}

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
