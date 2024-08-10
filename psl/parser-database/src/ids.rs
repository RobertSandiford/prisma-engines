use diagnostics::FileId;
use schema_ast::ast;

/// An AST identifier with the accompanyin file ID.
pub type InFile<Id> = (FileId, Id);

/// See [ast::ModelId]
pub type ModelIdInFile = InFile<ast::ModelId>;

/// See [ast::EnumId]
pub type EnumIdInFile = InFile<ast::EnumId>;

/// See [ast::ComputedEnumId]
pub type ComputedEnumIdInFile = InFile<ast::ComputedEnumId>;

/// See [ast::CompositeTypeId]
pub type CompositeTypeIdInFile = InFile<ast::CompositeTypeId>;

/// See [ast::TopId]
pub type TopIdInFile = InFile<ast::TopId>;

/// See [ast::AttributeId]
pub type AttributeIdInFile = InFile<ast::AttributeId>;

/// See [ast::AttributeContainer]
pub type AttributeContainerInFile = InFile<ast::AttributeContainer>;
