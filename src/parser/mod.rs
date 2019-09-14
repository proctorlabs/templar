mod rules;
mod tree;

use crate::*;
use pest::Parser;
use pest_derive::*;
use std::collections::BTreeMap;
use std::mem::replace;
use tree::ParseTree;

#[derive(Parser)]
#[grammar = "templar.pest"]
struct TemplarParser;

impl Templar {
    /// This is the non-generic form of parse()
    #[inline]
    pub fn parse_template(&self, input: &str) -> Result<Template> {
        let result: Node = self.parse_match(
            TemplarParser::parse(Rule::template_root, input)
                .map_err(|e| TemplarError::ParseFailure(format!("{}", e)))?,
        )?;
        Ok(result.set_operation(Operations::Concat).into())
    }

    /// Parses a string directly to a template
    #[inline]
    pub fn parse_expression(&self, input: &str) -> Result<Template> {
        Ok(self
            .parse_match(
                TemplarParser::parse(Rule::expression, input.trim())
                    .map_err(|e| TemplarError::ParseFailure(format!("{}", e)))?,
            )?
            .into())
    }
}
