use crate::model_loader::AIModel;
use crate::validator::Validator;

pub struct AIConsensus;

impl AIConsensus {
    /// Uses AI to dynamically select top-performing validators
    pub fn select_validators(validators: Vec<Validator>, network_load: u64) -> Vec<Validator> {
        let ai_model = AIModel::new();
        let validator_count = ai_model.predict_validator_reputation(50.0, network_load as f64);

        let mut sorted_validators = validators.clone();
        sorted_validators.sort_by_key(|v| v.reputation as u64);
        
        sorted_validators.into_iter().rev().take(validator_count as usize).collect()
    }
}
