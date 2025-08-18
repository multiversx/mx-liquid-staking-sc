use crate::Interact;

impl Interact {
    pub async fn deploy_governance_contract(&mut self) {
        self.governance_interactor
            .set_state(&self.governance_interactor.owner.to_address())
            .await;
        self.governance_interactor
            .set_state(&self.governance_interactor.user1.to_address())
            .await;
        self.governance_interactor
            .set_state(&self.governance_interactor.delegator.to_address())
            .await;

        let _ = self
            .governance_interactor
            .interactor
            .generate_blocks_until_epoch(8)
            .await;

        self.governance_interactor
            .proposal(
                &self.governance_interactor.owner.to_address(),
                "b29feffb6e80cb4622a5b9ee51793c7c2adef835",
                17,
                19,
            )
            .await;
        self.governance_interactor.view_config().await;
        self.governance_interactor.view_proposal(1).await;
    }
}
