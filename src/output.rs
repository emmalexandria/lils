mod entry;
mod long;
mod short;

use std::fmt::Display;

use crossterm::style::StyledContent;
pub use long::long;
pub use short::short;

pub struct MultiStyled<D>
where
    D: Display,
{
    sections: Vec<StyledContent<D>>,
}

impl<D: Display> From<StyledContent<D>> for MultiStyled<D> {
    fn from(value: StyledContent<D>) -> Self {
        Self {
            sections: vec![value],
        }
    }
}

impl<D: Display> Display for MultiStyled<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for sec in &self.sections {
            f.write_str(&sec.to_string())?;
        }

        Ok(())
    }
}

impl<D: Display> MultiStyled<D> {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }

    pub fn with(mut self, content: StyledContent<D>) -> Self {
        self.sections.push(content);
        self
    }

    pub fn push(&mut self, content: StyledContent<D>) {
        self.sections.push(content);
    }

    pub fn insert(&mut self, index: usize, content: StyledContent<D>) {
        self.sections.insert(index, content);
    }

    pub fn section(&self, index: usize) -> Option<&StyledContent<D>> {
        if index < self.sections.len() {
            return Some(&self.sections[index]);
        }

        return None;
    }

    pub fn output(&self) -> String {
        format!("{self}")
    }
}

impl<S: AsRef<str> + Display> MultiStyled<S> {
    pub fn len(&self) -> usize {
        self.sections
            .iter()
            .fold(0, |a, s| a + s.content().as_ref().len())
    }
}
