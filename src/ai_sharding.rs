use crate::model_loader::AIModel;
use crate::predictive_analytics::predict_shard_allocation;

pub struct AISharding {
    pub ai_model: AIModel,
}

impl AISharding {
    pub fn new(ai_model: AIModel) -> Self {
        Self { ai_model }
    }

    /// Uses AI to determine the optimal number of shards
    pub fn adapt_sharding(&self, network_load: u64) -> usize {
        let predicted_shards = predict_shard_allocation(network_load);
        predicted_shards
    }
}
