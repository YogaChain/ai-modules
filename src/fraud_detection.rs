use crate::model_loader::AIModel;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: String,
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub timestamp: u64,
}

pub struct FraudDetection {
    ai_model: AIModel,
}

impl FraudDetection {
    pub fn new(ai_model: AIModel) -> Self {
        Self { ai_model }
    }

    /// Uses AI model to detect fraudulent transactions
    pub fn detect_fraud(&self, transaction: &Transaction) -> bool {
        let risk_score = self.ai_model.predict_fraud_risk(transaction);
        risk_score > 0.75 // Flags transactions with high fraud probability
    }
}
