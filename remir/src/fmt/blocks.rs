//! Formatting for blocks

use std::fmt::Display;

use crate::{
    block::{Block, BlockReference},
    fmt::utils::fmt_list,
};

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.origins.is_empty() {
            let origins: Vec<&BlockReference> = self.origins.iter().collect();

            writeln!(f, "#origins: [{}]", fmt_list(&origins))?;
        }

        if !self.destinations.is_empty() {
            let destinations: Vec<&BlockReference> = self.destinations.iter().collect();

            writeln!(f, "#destinations [{}]", fmt_list(&destinations))?;
        }

        if !self.dependencies.is_empty() {
            let dependencies: Vec<&BlockReference> = self.dependencies.iter().collect();

            writeln!(f, "#dependencies [{}]", fmt_list(&dependencies))?;
        }

        if !self.variables.is_empty() {
            let variables: Vec<&String> = self.variables.keys().collect();

            writeln!(f, "#variables [{}]", fmt_list(&variables))?;
        }

        writeln!(f, "{}:", self.reference)?;

        for inst in &self.instructions {
            writeln!(f, "- {}", inst)?;
        }

        Ok(())
    }
}
