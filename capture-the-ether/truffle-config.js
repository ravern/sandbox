require('dotenv').config();

const HDWalletProvider = require('truffle-hdwallet-provider');
const infuraKey = process.env.INFURA_KEY;

const fs = require('fs');
const mnemonic = process.env.WALLET_MNEMONIC;

module.exports = {
  networks: {
    ropsten: {
      provider: () =>
        new HDWalletProvider(
          mnemonic,
          `https://ropsten.infura.io/v3/${infuraKey}`,
        ),
      network_id: 3,
      gas: 5500000,
      skipDryRun: true,
    },
  },
  compilers: {
    solc: {
      version: '^0.4.21',
    },
  },
};
