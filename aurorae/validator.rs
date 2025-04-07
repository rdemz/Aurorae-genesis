use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    Invalid(String),
    Warning(String),
    NotApplicable,
}

pub struct ValidationRule {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub priority: u8,
    pub validator: Box<dyn Fn(&str) -> ValidationResult + Send + Sync>,
}

pub struct Validator {
    pub rules: HashMap<Uuid, ValidationRule>,
}

impl Validator {
    pub fn new() -> Self {
        let mut validator = Self {
            rules: HashMap::new(),
        };
        
        validator.add_default_rules();
        validator
    }
    
    fn add_default_rules(&mut self) {
        // Règle: non vide
        self.add_rule(
            "Non vide",
            "Vérifie que la valeur n'est pas vide",
            10,
            Box::new(|value| {
                if value.trim().is_empty() {
                    ValidationResult::Invalid("La valeur ne peut pas être vide".to_string())
                } else {
                    ValidationResult::Valid
                }
            })
        );
        
        // Règle: longueur maximum
        self.add_rule(
            "Longueur maximum",
            "Vérifie que la valeur ne dépasse pas 1024 caractères",
            5,
            Box::new(|value| {
                if value.len() > 1024 {
                    ValidationResult::Invalid(format!(
                        "La valeur dépasse la longueur maximum ({})",
                        value.len()
                    ))
                } else {
                    ValidationResult::Valid
                }
            })
        );
        
        // Règle: caractères valides
        self.add_rule(
            "Caractères valides",
            "Vérifie que la valeur ne contient pas de caractères dangereux",
            8,
            Box::new(|value| {
                if value.contains('<') && value.contains('>') {
                    ValidationResult::Warning(
                        "La valeur contient potentiellement des balises HTML".to_string()
                    )
                } else {
                    ValidationResult::Valid
                }
            })
        );
    }
    
    pub fn add_rule(
        &mut self, 
        name: &str,
        description: &str,
        priority: u8,
        validator: Box<dyn Fn(&str) -> ValidationResult + Send + Sync>,
    ) -> Uuid {
        let rule_id = Uuid::new_v4();
        
        let rule = ValidationRule {
            id: rule_id,
            name: name.to_string(),
            description: description.to_string(),
            priority,
            validator,
        };
        
        self.rules.insert(rule_id, rule);
        rule_id
    }
    
    pub fn validate(&self, value: &str) -> Vec<(Uuid, ValidationResult)> {
        let mut results = Vec::new();
        
        // Trier les règles par priorité (décroissante)
        let mut rules: Vec<&ValidationRule> = self.rules.values().collect();
        rules.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        // Appliquer chaque règle
        for rule in rules {
            let result = (rule.validator)(value);
            results.push((rule.id, result));
        }
        
        results
    }
    
    pub fn is_valid(&self, value: &str) -> bool {
        let validation_results = self.validate(value);
        
        !validation_results
            .iter()
            .any(|(_, result)| matches!(result, ValidationResult::Invalid(_)))
    }
    
    pub fn validate_with_context(&self, value: &str, context: &str) -> (bool, Vec<String>) {
        let mut is_valid = true;
        let mut messages = Vec::new();
        
        for (rule_id, result) in self.validate(value) {
            match result {
                ValidationResult::Invalid(msg) => {
                    let rule_name = self.rules.get(&rule_id).map(|r| r.name.clone()).unwrap_or_default();
                    messages.push(format!("Erreur ({}): {}", rule_name, msg));
                    is_valid = false;
                },
                ValidationResult::Warning(msg) => {
                    let rule_name = self.rules.get(&rule_id).map(|r| r.name.clone()).unwrap_or_default();
                    messages.push(format!("Avertissement ({}): {}", rule_name, msg));
                },
                _ => {}
            }
        }
        
        (is_valid, messages)
    }
}
