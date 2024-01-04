# trn-nut-rs

The TRN permission domain set and codec.  
Intended for use with Doughnuts on TRN to provide safe, delegated transactions.  

The formal spec. is available [here](https://github.com/cennznet/doughnut-paper/blob/master/CENNZnet_format.md)  

## Generate JS/Wasm bindings

This crate also generates an npm package <to be added>
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

