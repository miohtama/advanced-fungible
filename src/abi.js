// Hand-written ABI methods for the contracts
export const abi = {
    pool: {
        viewMethods: ['get_total_received', 'is_receiver'],
        changeMethods: ['new', 'on_token_received']
    },

    token: {
        viewMethods: ['get_total_supply', 'get_balance', 'get_locked_balance', 'get_rollback_count'],
        changeMethods: ['new', 'send', 'process_bytes']
    }
};