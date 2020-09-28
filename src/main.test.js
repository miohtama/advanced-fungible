
beforeAll(async () => {
  nearjs = await testUtils.setUpTestConnection();
  workingAccount = await testUtils.createAccount(nearjs);
});

describe('with promises', () => {
  let contract, contract1, contract2;
  let oldLog;
  let logs;
  let contractName = testUtils.generateUniqueString('cnt');
  let contractName1 = testUtils.generateUniqueString('cnt');
  let contractName2 = testUtils.generateUniqueString('cnt');

  beforeAll(async () => {
      contract = await testUtils.deployContract(workingAccount, contractName, 'token');
  });

  beforeEach(async () => {
      oldLog = console.log;
      logs = [];
      console.log = function() {
          logs.push(Array.from(arguments).join(' '));
      };
  });

  afterEach(async () => {
      console.log = oldLog;
  });

  // -> means async call
  // => means callback

  test('single promise, no callback (A->B)', async () => {
      const realResult = await contract.callPromise({args: {
          receiver: contractName1,
          methodName: 'callbackWithName',
          args: null,
          gas: '3000000000000',
          balance: '0',
          callback: null,
          callbackArgs: null,
          callbackBalance: '0',
          callbackGas: '0',
      }}, CONTRACT_CALL_GAS);

      expect(realResult).toEqual("333");
  });


});
