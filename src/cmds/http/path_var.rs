use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::iter::Iterator;

/// path variable extractor
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct PathVarExtractor {
    num_segments: usize,
    vars_map: BTreeMap<usize, String>,
}

impl PathVarExtractor {
    pub fn new(uri: &str) -> Option<PathVarExtractor> {
        let segments = uri.split("/").map(str::to_owned).collect::<Vec<String>>();
        let size = segments.len();
        // variables map from index in segments to name
        let mut vars = BTreeMap::new();
        segments
            .iter()
            .enumerate()
            .filter(|(_, v)| v.starts_with(":"))
            .for_each(|(idx, v)| {
                vars.insert(idx, v.trim_start_matches(":").to_owned());
            });

        if vars.len() > 0 {
            Some(PathVarExtractor {
                num_segments: size,
                vars_map: vars,
            })
        } else {
            None
        }
    }

    pub fn extract_vars(&self, uri: &str) -> BTreeMap<String, String> {
        let mut vars = BTreeMap::new();
        let segments = uri.split("/").collect::<Vec<&str>>();
        if segments.len() == self.num_segments {
            for (idx, v) in self.vars_map.iter() {
                vars.insert(v.to_owned(), segments[*idx].to_owned());
            }
        }
        vars
    }

    pub fn unify(&self, uri: &str) -> String {
        uri.split("/")
            .enumerate()
            .map(|(ref idx, v)| {
                if let Some(_) = self.vars_map.get(idx) {
                    "{}"
                } else {
                    v
                }
            })
            .collect::<Vec<&str>>()
            .join("/")
    }
}

mod test {
    use super::*;

    #[test]
    fn return_no_extractor_when_no_variable_in_uri() {
        let uri = "/api/v1/list";
        let result = PathVarExtractor::new(uri);
        assert_eq!(None, result);
    }

    #[test]
    fn create_extractor_successfully() {
        let uri = "/api/:ver/list/:id";
        let result = PathVarExtractor::new(uri);
        let extractor = result.unwrap();
        let vars = extractor.extract_vars("/api/v1/list/1");
        assert_eq!(2, vars.len());
        assert_eq!(Some("v1"), vars.get("ver").map(String::as_ref));
        assert_eq!(Some("1"), vars.get("id").map(String::as_ref))
    }
}
