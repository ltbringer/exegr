extern crate regex;

use pyo3::prelude::*;
use std::os::raw::c_double;
use regex::Regex;


#[pyclass]
struct GroupRegexMatch {
  compiled_patterns: Vec<Regex>
}

#[pymethods]
impl GroupRegexMatch {
  #[new]
  fn new(patterns: Vec<&str>) -> PyResult<GroupRegexMatch> {
    let compiled_patterns: Vec<Regex> = patterns.into_iter()
        .map(|pattern| Regex::new(pattern).unwrap())
        .collect();
    Ok(GroupRegexMatch { compiled_patterns })
  }

  fn search(&self, texts: Vec<&str>) -> PyResult<Vec<c_double>> {
    Ok(texts.iter()
      .map(|text| {
        self.compiled_patterns.iter()
          .map(|pattern| {
            match pattern.find(text) {
              None => 0.0,
              Some(mat) => ((mat.end() - mat.start())
                as c_double / text.len() as c_double)
            }
          }).fold(-1./0., f64::max)
      }).collect())
  }
}


#[pymodule]
fn exegr(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_class::<GroupRegexMatch>()?;
  Ok(())
}

// #[cfg(test)]
// mod tests;