import { abi } from './abi';
import { createAccount, setUpTestConnection, deployContract } from './test-utils';

// import fetch
global.fetch = require("node-fetch");

let near;

// Contract accounts
let poolContract, tokenContract, anotherTokenContract;

// Normal user accounts
let vitalik, gavin, deployer;

beforeAll(async function () {

    near = await setUpTestConnection();
    vitalik = await createAccount(near);
    gavin = await createAccount(near);
    deployer = await createAccount(near);

    poolContract = await deployContract(deployer, 'pool', 'pool', abi.pool);
    tokenContract = await deployContract(deployer, 'tokenContract', 'token', abi.token);
    anotherTokenContract = await deployContract(deployer, 'anotherTokenContract', 'token', abi.token);

});

test('Create pool', async () => {

})
