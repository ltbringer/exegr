extern crate regex;

use pyo3::prelude::*;
use std::os::raw::c_double;
use regex::{Regex};


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

  pub fn search(&self, texts: Vec<&str>) -> PyResult<Vec<f64>> {
    Ok(self.compiled_patterns.iter()
      .map(|pattern| {
        let scores: Vec<f64> = texts.iter()
          .map(|text| {
            match pattern.find(text) {
              None => 0.0,
              Some(match_) => ((match_.end() - match_.start())
                as c_double / text.len() as c_double)
            }
          }).collect();

        let non_zero_scores: Vec<f64> = scores.iter()
          .cloned()
          .filter(|score| score > &0.0)
          .collect();

        let max_score: f64 = non_zero_scores.iter()
          .cloned()
          .fold(-1./0., f64::max);

        let score = -1.0 * (non_zero_scores.len() as f64) * max_score;
        let e_to_score = score.exp();
        let score_sigmoid: f64 = if non_zero_scores.len() > 0 { 1. / (1. + e_to_score) } else { 0. };
        score_sigmoid
      }).collect())
  }

  fn span_matrix(&self, texts: Vec<&str>) -> PyResult<Vec<Vec<Option<(usize, usize)>>>> {
    let mut span_matrix: Vec<Vec<Option<(usize, usize)>>> = Vec::with_capacity(self.compiled_patterns.len());
    for pattern in self.compiled_patterns.iter() {
      let mut span_vector: Vec<Option<(usize, usize)>> = Vec::with_capacity(texts.len());
      for text in texts.iter() {
        match pattern.find(text) {
          None => span_vector.push(None),
          Some(mat) => span_vector.push(Some((mat.start(), mat.end())))
        };
      }
      span_matrix.push(span_vector);
    }
    Ok(span_matrix)
  }
}


#[pymodule]
fn exegr(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_class::<GroupRegexMatch>()?;
  Ok(())
}

// #[cfg(test)]
// mod tests;