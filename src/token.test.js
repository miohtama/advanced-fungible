import BN from 'bn.js';
import { abi } from './abi';
import { createAccount, setUpTestConnection, deployContract, generateUniqueString } from './test-utils';

const TRANSFER_GAS = new BN("300000000000000");

// NEAR connection
let near;

// Normal user accounts
let deployer, vitalik, gavin;

beforeAll(async function () {
    near = await setUpTestConnection();
    deployer = await createAccount(near);
    vitalik = await createAccount(near);
    gavin = await createAccount(near);
});


test('Deploy token contract', async () => {
    const tokenContract = await deployContract(deployer, generateUniqueString('cnt'), 'token', abi.token);

    await tokenContract.new({
        // Vitalik owns us
        owner_id: vitalik.accountId,
        total_supply: 10000,
    });

    const totalSupply = await tokenContract.get_total_supply();
    expect(totalSupply).toEqual(10000);

    // The initial owner has everything
    const balance = await tokenContract.get_balance({ owner_id: vitalik.accountId });
    expect(balance).toEqual(10000);

    const locked = await tokenContract.get_locked_balance({ owner_id: vitalik.accountId });
    expect(locked).toEqual(0);

    // No balance
    const balance2 = await tokenContract.get_balance({ owner_id: gavin.accountId });
    expect(balance2).toEqual(0);

    // No balance
    const balance3 = await tokenContract.get_balance({ owner_id: deployer.accountId });
    expect(balance3).toEqual(0);

});


test('Can send between accounts', async () => {

    const tokenContract = await deployContract(deployer, generateUniqueString('cnt'), 'token', abi.token);

    await tokenContract.new({
        // Vitalik owns us
        owner_id: vitalik.accountId,
        total_supply: 10000,
    });

    // Vitalik calls token.send()
    const result = await vitalik.functionCall(
        tokenContract.contractId,
        "send",
        {
            new_owner_id: gavin.accountId,
            amount: 800,
            message: [],
            notify: false
        },
        TRANSFER_GAS,
    )

    expect(result.status?.SuccessValue).toBe('');

    // The initial owner has everything
    const balance = await tokenContract.get_balance({ owner_id: vitalik.accountId });
    expect(balance).toEqual(9200);

    const balance2 = await tokenContract.get_balance({ owner_id: gavin.accountId });
    expect(balance2).toEqual(800);

    // No balance
    const balance3 = await tokenContract.get_balance({ owner_id: deployer.accountId });
    expect(balance3).toEqual(0);
});


test('Cannot send too much', async () => {

    const tokenContract = await deployContract(deployer, generateUniqueString('cnt'), 'token', abi.token);

    await tokenContract.new({
        // Vitalik owns us
        owner_id: vitalik.accountId,
        total_supply: 10000,
    });

    try {
        await vitalik.functionCall(
            tokenContract.contractId,
            "send",
            {
                new_owner_id: gavin.accountId,
                amount: 11000,
                message: [],
                notify: false
            }
        )
        throw new Error("Not reached");
    } catch(e) {
        expect(e.panic_msg).toMatch(/Not enough balance/);
    }
});