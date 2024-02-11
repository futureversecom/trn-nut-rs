# @trn/trnnut-wasm

Wasm TRNNut codec and maker.
Currently compliant with the version 0 spec.  

## Create a TRNNut (unsigned)

```js
const TRNNut = require('@trn/trnnut-wasm').default;

// Method is a vector of methods - Vec<Method>
const methods = [
  {
    name: "test_method_check11",
    block_cooldown: 270549120,
    constraints: null,
  },
];

// Modules is a vector of modules -  Vec<Module>
const modules = [
  {
    name: "test_module_check1",
    block_cooldown: 270549120,
    methods: [
      {
        name: "test_method_check11",
        block_cooldown: 270549120,
        constraints: null,
      }
    ],
  },
];

return new TRNNut(modules);
```

## Inspect TRNNut Fields

Getter functions for inspecting a TRNNut

```js
const trnnut = new TRNNut(...);
module = trnnut.getModule("module_test"); // returns the module with name 'module_test' if exist else returns undefined
```

## TRNNut Encoding and Decoding

`Encoding`: Encode a trnnut object

`Decoding`: Create a trnnut object from a encoded trnnut

```js
const TRNNut = require('@trn/trnnut-wasm').default;

const payload = [0, 0, 0, 1, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, ..., 0, 0, 0, 0, 0, 0, 0, 0, 0];
const trnnut = TRNNut.decode(payload);

const trnnut = new TRNNut(...);
const encoded = trnnut.encode();
```
