import { abi } from './abi';
import { createAccount, setUpTestConnection, deployContract, generateUniqueString } from './test-utils';


let near;

// Normal user accounts
let deployer;

beforeAll(async function () {
    near = await setUpTestConnection();
    deployer = await createAccount(near);
});

test('Deploy pool contract', async () => {
    const poolContract = await deployContract(deployer, generateUniqueString('cnt'), 'pool', abi.pool);

    let action = await deployer.functionCall(
        poolContract.contractId,
        "new",
        {
            // Can be any account in this test
            token_id: deployer.accountId,
        }
    );
    expect(action.status?.SuccessValue).toBe('');

    const received = await poolContract.get_total_received();
    expect(received).toEqual(0);
});


test('Cannot initialize pool twice', async () => {
    const poolContract = await deployContract(deployer, generateUniqueString('cnt'), 'pool', abi.pool);
    await poolContract.new({ token_id: deployer.accountId });
    // second init
    try {
        await poolContract.new({ token_id: deployer.accountId });
        throw new Error('Not reachable');
    } catch(e) {
        expect(e.panic_msg).toMatch(/Already initialized/);
    }
});
