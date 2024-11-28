multiversx_sc::imports!();

use crate::{contexts::base::StorageCache, liquidity_pool};

#[multiversx_sc::module]
pub trait ViewsModule:
    crate::config::ConfigModule
    + multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + liquidity_pool::LiquidityPoolModule
{
    // views
    #[view(getLsValueForPosition)]
    fn get_ls_value_for_position(&self, ls_token_amount: BigUint) -> BigUint {
        let mut storage_cache = StorageCache::new(self);
        storage_cache.skip_commit = true;
        self.get_egld_amount(&ls_token_amount, &storage_cache)
    }
}
