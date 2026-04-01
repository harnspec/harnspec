//! Validate tool — validate specs for issues

use harnspec_core::SpecLoader;
use serde_json::{json, Value};

pub(crate) fn tool_validate(specs_dir: &str, args: Value) -> Result<String, String> {
    let loader = SpecLoader::new(specs_dir);
    let specs = loader.load_all().map_err(|e| e.to_string())?;

    let fm_validator = harnspec_core::FrontmatterValidator::new();
    let struct_validator = harnspec_core::StructureValidator::new();
    let token_validator = harnspec_core::TokenCountValidator::new();

    let mut validation_errors = Vec::new();

    let specs_to_validate = if let Some(spec_path) = args.get("specPath").and_then(|v| v.as_str()) {
        let spec = loader
            .load(spec_path)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("Spec not found: {}", spec_path))?;
        vec![spec]
    } else {
        specs.clone()
    };

    for spec in &specs_to_validate {
        let mut result = harnspec_core::ValidationResult::new(&spec.path);
        result.merge(fm_validator.validate(spec));
        result.merge(struct_validator.validate(spec));
        result.merge(token_validator.validate(spec));

        if result.has_errors() || result.has_warnings() {
            validation_errors.push(json!({
                "spec": spec.path,
                "errors": result.errors().map(|i| i.message.clone()).collect::<Vec<_>>(),
                "warnings": result.warnings().map(|i| i.message.clone()).collect::<Vec<_>>(),
            }));
        }
    }

    if validation_errors.is_empty() {
        Ok(format!(
            "All {} specs passed validation",
            specs_to_validate.len()
        ))
    } else {
        serde_json::to_string_pretty(&json!({
            "total": specs_to_validate.len(),
            "errors": validation_errors
        }))
        .map_err(|e| e.to_string())
    }
}

pub(crate) fn get_definition() -> crate::protocol::ToolDefinition {
    crate::protocol::ToolDefinition {
        name: "validate".to_string(),
        description: "Validate specs for issues (frontmatter, structure, dependencies)".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "specPath": {
                    "type": "string",
                    "description": "Specific spec to validate (validates all if not provided)"
                }
            }
        }),
    }
}
