const { test } = require("shelljs");

beforeAll(async () => {
});

describe('with promises', () => {

    test('no test', async() => {

    });

  /*
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
  */

});
