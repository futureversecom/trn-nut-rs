# trnnut-rs

NOTE: This respository has been archived due to the fact that the functionality of TRNNut has been merged with the [TRN-Doughnut](https://github.com/futureversecom/trn-doughnut-rs) repository - this [PR](https://github.com/futureversecom/trn-doughnut-rs/pull/6) was responsible for the merge.

All future developments of the trnnut and doughnuts will continue in the doughnut repository.

---

The TRN permission domain set and codec.  
Intended for use with Doughnuts on TRN to provide safe, delegated transactions.  

The formal spec. is available [here](https://github.com/cennznet/doughnut-paper/blob/master/CENNZnet_format.md)  

## Generate JS/Wasm bindings

This crate also generates an npm package [@trn/trnnut-wasm](https://www.npmjs.com/package/@trn/trnnut-wasm)
using [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/).

To generate the package run:
```bash
# install wasm pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# build
cd js/ && yarn build

# Run tests
yarn test
```

