import { abi } from './abi';
import { createAccount, setUpTestConnection, deployContract } from './test-utils';

let near;

// Contract accounts
let poolContract, tokenContract, anotherTokenContract;

// Normal user accounts
let vitalik, gavin;

beforeAll(async function () {

    near = setUpTestConnection();

    vitalik = createAccount(near);
    gavin = createAccount(near);

    poolContract = deployContract('pool', 'pool', abi.pool);
    tokenContract = deployContract('tokenContract', 'token', abi.token);
    anotherTokenContract = deployContract('anotherTokenContract', 'token', abi.token);

});

test('Create pool', async () => {

})
