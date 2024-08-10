// use super::{Attribute, Comment, Identifier, Span, WithAttributes, WithDocumentation, WithIdentifier, WithSpan};

// /// An opaque identifier for a value in an AST enum. Use the
// /// `r#enum[enum_value_id]` syntax to resolve the id to an `ast::ComputedEnumValue`.
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct ComputedEnumValueId(pub(super) u32);

// impl ComputedEnumValueId {
//     /// Used for range bounds when iterating over BTreeMaps.
//     pub const MIN: ComputedEnumValueId = ComputedEnumValueId(0);
//     /// Used for range bounds when iterating over BTreeMaps.
//     pub const MAX: ComputedEnumValueId = ComputedEnumValueId(u32::MAX);
// }

// impl std::ops::Index<ComputedEnumValueId> for ComputedEnum {
//     type Output = ComputedEnumValue;

//     fn index(&self, index: ComputedEnumValueId) -> &Self::Output {
//         &self.values[index.0 as usize]
//     }
// }

// /// A computed enum declaration.
// #[derive(Debug, Clone)]
// pub struct ComputedEnum {
//     /// The name of the enum.
//     ///
//     /// ```ignore
//     /// enum Foo { ... }
//     ///      ^^^
//     /// ```
//     //pub(crate) name: Identifier,
//     /// The values of the enum.
//     ///
//     /// ```ignore
//     /// enum Foo {
//     ///   Value1
//     ///   ^^^^^^
//     ///   Value2
//     ///   ^^^^^^
//     /// }
//     /// ```
//     pub values: Vec<ComputedEnumValue>,
//     /// The attributes of this enum.
//     ///
//     /// ```ignore
//     /// enum Foo {
//     ///   Value1
//     ///   Value2
//     ///
//     ///   @@map("1Foo")
//     ///   ^^^^^^^^^^^^^
//     /// }
//     /// ```
//     //pub attributes: Vec<Attribute>,
//     /// The comments for this enum.
//     ///
//     /// ```ignore
//     /// /// Lorem ipsum
//     ///     ^^^^^^^^^^^
//     /// enum Foo {
//     ///   Value1
//     ///   Value2
//     /// }
//     /// ```
//     pub(crate) documentation: Option<Comment>,
//     /// The location of this enum in the text representation.
//     pub span: Span,
//     /// The span of the inner contents.
//     pub inner_span: Span,
// }

// impl ComputedEnum {
//     pub fn iter_values(&self) -> impl ExactSizeIterator<Item = (ComputedEnumValueId, &ComputedEnumValue)> {
//         self.values
//             .iter()
//             .enumerate()
//             .map(|(idx, field)| (ComputedEnumValueId(idx as u32), field))
//     }
// }

// impl WithIdentifier for ComputedEnum {
//     fn identifier(&self) -> &Identifier {
//         &self.name
//     }
// }

// impl WithSpan for ComputedEnum {
//     fn span(&self) -> Span {
//         self.span
//     }
// }

// impl WithAttributes for ComputedEnum {
//     fn attributes(&self) -> &[Attribute] {
//         &self.attributes
//     }
// }

// impl WithDocumentation for ComputedEnum {
//     fn documentation(&self) -> Option<&str> {
//         self.documentation.as_ref().map(|doc| doc.text.as_str())
//     }
// }

// /// An enum value definition.
// #[derive(Debug, Clone)]
// pub struct ComputedEnumValue {
//     /// The name of the enum value as it will be exposed by the api.
//     pub name: Identifier,
//     /// The attributes of this value.
//     ///
//     /// ```ignore
//     /// yellow @map("orange")
//     ///        ^^^^^^^^^^^^^^
//     /// ```
//     pub attributes: Vec<Attribute>,
//     pub(crate) documentation: Option<Comment>,
//     /// The location of this enum value in the text representation.
//     pub span: Span,
// }

// impl WithIdentifier for ComputedEnumValue {
//     fn identifier(&self) -> &Identifier {
//         &self.name
//     }
// }

// impl WithAttributes for ComputedEnumValue {
//     fn attributes(&self) -> &[Attribute] {
//         &self.attributes
//     }
// }

// impl WithSpan for ComputedEnumValue {
//     fn span(&self) -> Span {
//         self.span
//     }
// }

// impl WithDocumentation for ComputedEnumValue {
//     fn documentation(&self) -> Option<&str> {
//         self.documentation.as_ref().map(|doc| doc.text.as_str())
//     }
// }
