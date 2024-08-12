use super::{Identifier, Span, WithSpan};
use std::fmt::{Display, Formatter};

/// A list of arguments inside parentheses.
#[derive(Debug, Clone, Default)]
pub struct ComputedTypeArgumentsList {
    /// The arguments themselves.
    ///
    /// ```ignore
    /// @@index([a, b, c], map: "myidix")
    ///         ^^^^^^^^^^^^^^^^^^^^^^^^
    /// ```
    pub arguments: Vec<ComputedTypeArgument>,
    /// The arguments without a value:
    ///
    /// ```ignore
    /// @default("george", map: )
    ///                    ^^^^
    /// ```
    //pub empty_arguments: Vec<EmptyArgument>,
    /// The trailing comma at the end of the arguments list.
    ///
    /// ```ignore
    /// @relation(fields: [a, b], references: [id, name], )
    ///                                                 ^
    /// ```
    pub trailing_comma: Option<Span>,
}

impl ComputedTypeArgumentsList {
    pub(crate) fn iter(&self) -> std::slice::Iter<'_, ComputedTypeArgument> {
        self.arguments.iter()
    }
}

/// An argument, either for attributes or for function call expressions.
#[derive(Debug, Clone)]
pub struct ComputedTypeArgument {
    /// The argument name, if applicable.
    ///
    /// ```ignore
    /// @id(map: "myIndex")
    ///     ^^^
    /// ```
    pub name: Option<Identifier>,
    /// The argument value.
    ///
    /// ```ignore
    /// @id("myIndex")
    ///     ^^^^^^^^^
    /// ```
    pub value: Identifier,
    /// Location of the argument in the text representation.
    pub span: Span,
}

impl Display for ComputedTypeArgument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            f.write_str(&name.name)?;
            f.write_str(": ")?;
        }
        Display::fmt(&self.value.name, f)
    }
}

impl ComputedTypeArgument {
    pub fn is_unnamed(&self) -> bool {
        self.name.is_none()
    }

    pub fn name(&self) -> Option<&str> {
        match &self.name {
            Some(ident) => Some(ident.name.as_str()),
            None => None,
        }
    }
}

impl WithSpan for ComputedTypeArgument {
    fn span(&self) -> Span {
        self.span
    }
}

