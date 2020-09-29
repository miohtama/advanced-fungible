// Hand-written ABI methods for the contracts
export const abi = {
    pool: {
        viewMethods: ['get_total_received'],
        changeMethods: ['new', 'on_token_received']
    },

    token: {
        changeMethods: ['send']
    }
};