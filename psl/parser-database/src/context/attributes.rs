use super::*;
use crate::interner::StringId;

#[derive(Default, Debug)]
pub(super) struct AttributesValidationState {
    /// The attributes list being validated.
    pub(super) attributes: Option<crate::AttributeContainerInFile>,
    pub(super) unused_attributes: HashSet<crate::AttributeIdInFile>, // the _remaining_ attributes

    /// The attribute being validated.
    pub(super) attribute: Option<crate::AttributeIdInFile>,
    pub(super) args: HashMap<Option<StringId>, usize>, // the _remaining_ arguments of `attribute`
}

impl AttributesValidationState {
    pub(super) fn set_attributes(&mut self, attributes: crate::AttributeContainerInFile, ast: &ast::SchemaAst) {
        let file_id = attributes.0;
        let attribute_ids =
            (0..ast[attributes.1].len()).map(|idx| (file_id, ast::AttributeId::new_in_container(attributes.1, idx)));
        self.unused_attributes.clear();
        self.unused_attributes.extend(attribute_ids);

        self.attributes = Some(attributes);
    }
}
