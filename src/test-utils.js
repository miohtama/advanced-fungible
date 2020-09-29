const fs = require('fs').promises;
const BN = require('bn.js');
const nearApi = require('near-api-js');
const assert = require('assert');
const { resolve } = require("path");


const networkId = 'unittest';

// TODO: Cargo cult copying because I have not idea what should be there
// Some deployments take more gas. If the deployment fails just keep increasing this number.
const CONTRACT_BALANCE = new BN('50000000000000000000000000');

async function setUpTestConnection() {
    const keyStore = new nearApi.keyStores.InMemoryKeyStore();
    const config = Object.assign(require('./config')(process.env.NODE_ENV || 'test'), {
        networkId: networkId,
        deps: { keyStore },
    });

    // TODO: I have no clue what this is because it was commented in the original source code.
    if (config.masterAccount) {
        await keyStore.setKey(networkId, config.masterAccount, nearApi.utils.KeyPair.fromString('ed25519:2wyRcSwSuHtRVmkMCGjPwnzZmQLeXLzLLyED1NDMt4BjnKgQL6tF85yBx6Jr26D2dUNeC716RBoTxntVHsegogYw'));
    }

    return nearApi.connect(config);
}

// Generate some unique string with a given prefix using the alice nonce.
function generateUniqueString(prefix) {
    return `${prefix}-${Date.now()}-${Math.round(Math.random() * 1000000)}`;
}

async function createAccount(near) {
    assert(near);
    assert(near.connection);
    assert(near.connection.signer);
    const newAccountName = generateUniqueString('test');
    const newPublicKey = await near.connection.signer.createKey(newAccountName, networkId);
    await near.createAccount(newAccountName, newPublicKey);
    const account = new nearApi.Account(near.connection, newAccountName);
    return account;
}

async function deployContract(workingAccount, contractId, contractName, abi) {
    const newPublicKey = await workingAccount.connection.signer.createKey(contractId, networkId);
    let path = `${__dirname}/../contract/target/wasm32-unknown-unknown/release/nep9000_${contractName}.wasm`;
    path = resolve(path);
    const data = [...(await fs.readFile(path))];

    assert(data.length < 200000, "That's one massive contract");

    await workingAccount.createAndDeployContract(contractId, newPublicKey, data, CONTRACT_BALANCE);
    const contract = new nearApi.Contract(workingAccount, contractId, abi);
    contract.contractId = contractId;
    return contract;
}

function sleep(time) {
    return new Promise(function (resolve) {
        setTimeout(resolve, time);
    });
}

async function ensureDir(dirpath) {
    try {
        await fs.mkdir(dirpath, { recursive: true });
    } catch (err) {
        if (err.code !== 'EEXIST') throw err;
    }
}

module.exports = {
    setUpTestConnection,
    networkId,
    generateUniqueString,
    createAccount,
    deployContract,
    sleep,
    ensureDir
};
