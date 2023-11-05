#[async_trait::async_trait]
pub trait AgentUnit {
    type Target: EntityEnum;
    
    async fn invoke(&self, surface: &mut AgentSurface);
}