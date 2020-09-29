// Hand-written ABI methods for the contracts
export const abi = {
    pool: {
        viewMethods: ['get_total_received'],
        changeMethods: ['setValue', 'callPromise']
    },

    token: {
        changeMethods: ['send']
    }
};