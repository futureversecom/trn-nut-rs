const TRNNut = require('../libNode/trnnut').TRNNut;

// The test used is same as it_works_decode_with_method_cooldown in rust
let encodedTRNNut = new Uint8Array([
    0, 0, 0, 1, 109, 111, 100, 117, 108, 101, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 81, 1, 0, 0, 109, 101, 116, 104, 111,
    100, 95, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0
]);

describe("wasm trnnut", () => {
  test("it decodes and verifies", () => {
    let trnnut = TRNNut.decode(encodedTRNNut);
    expect(trnnut.encode()).toEqual(encodedTRNNut);
    let module = trnnut.getModule("module_test");
    expect(module.name).toEqual('module_test');
    expect(module.block_cooldown).toEqual(86400);
    expect(module.methods[0]).toContain("method_test");
  });

  test ("create instance of trnnut", () => {
    const contract_address = new Uint8Array([
      27, 137,  65,  29, 182,  25, 157,  61,
      226,  13, 230,  14, 111,   6,  25, 186,
      227, 117, 177, 244, 172, 147,  40, 119,
      209,  78,  13, 109, 236, 119, 205, 202
    ]);

    const module = [
        [
            "test_module_check1",  {
                "name":"test_module_check1",
                "block_cooldown":270549120,
                "methods":[
                    [
                        "test_method_check1",  {
                                      "name":"test_method_check11",
                                      "block_cooldown":270549120,
                                      "constraints":null
                                      }
                    ],
                    [
                        "test_method_check2", {
                              "name":"test_method_check12",
                              "block_cooldown":270545024,
                              "constraints":null
                          }
                    ]
                ]
            }
        ],
      [
        "test_module_check2",  {
        "name":"test_module_check2",
        "block_cooldown":270541120,
        "methods":[
          [
            "test_method_check2",  {
            "name":"test_method_check21",
            "block_cooldown":270541120,
            "constraints":null
          }
          ]
        ]
      }
      ],
    ];

    const contract  = [[[27,137,65,29,182,25,157,61,226,13,230,14,111,6,25,186,227,117,177,244,172,147,40,119,209,78,13,109,236,119,205,202],{"address":[27,137,65,29,182,25,157,61,226,13,230,14,111,6,25,186,227,117,177,244,172,147,40,119,209,78,13,109,236,119,205,202],"block_cooldown":270549120}]];
    let trnnutNew = new TRNNut(module, contract);
    let extract_module = trnnutNew.getModule("test_module_check1");
    expect(extract_module.name).toEqual('test_module_check1');
    expect(extract_module.block_cooldown).toEqual(270549120);
    expect(extract_module.methods[0]).toContain("test_method_check1");
    let extract_contract = trnnutNew.getContract(contract_address);
    expect(extract_contract.block_cooldown).toEqual(270549120);
    expect(trnnutNew.verifyContract(contract_address)).toEqual(true);
  });

    test("test when module do not exist", () => {
        let trnnut = TRNNut.decode(encodedTRNNut);
        let module = trnnut.getModule("module_test1");
        expect(module).toEqual(undefined);
        const contract_address = new Uint8Array([
            27, 137,  65,  29, 182,  25, 157,  61,
            226,  13, 230,  14, 111,   6,  25, 186,
            227, 117, 177, 244, 172, 147,  40, 119,
            209,  78,  13, 109, 236, 119, 205, 202
        ]);
        let contract = trnnut.getContract(contract_address);
        expect(contract).toEqual(undefined);
    });
    test ("create instance of trnnut with constraint payload", () => {
        const contract_address = new Uint8Array([
            27, 137,  65,  29, 182,  25, 157,  61,
            226,  13, 230,  14, 111,   6,  25, 186,
            227, 117, 177, 244, 172, 147,  40, 119,
            209,  78,  13, 109, 236, 119, 205, 202
        ]);

        const module = [
            [
                "Balances",  {
                "name":"Balances",
                "block_cooldown":0,
                "methods":[
                    [
                        "transfer",  {
                        "name":"transfer",
                        "block_cooldown":0,
                        "constraints":[...contract_address]
                    }
                    ],
                ]
            }
            ],
        ];
        const contract  = [[[27,137,65,29,182,25,157,61,226,13,230,14,111,6,25,186,227,117,177,244,172,147,40,119,209,78,13,109,236,119,205,202],{"address":[27,137,65,29,182,25,157,61,226,13,230,14,111,6,25,186,227,117,177,244,172,147,40,119,209,78,13,109,236,119,205,202],"block_cooldown":270549120}]];
        const trnnut = new TRNNut(module, contract);

        let extract_module = trnnut.getModule("Balances");
        expect(extract_module.name).toEqual('Balances');
        expect(extract_module.block_cooldown).toEqual(0);
        expect(extract_module.methods[0]).toContain("transfer");
        // expect(extract_module.methods[0].constraints).toEqual(contract_address)
        let extract_contract = trnnut.getContract(contract_address);
        expect(extract_contract.block_cooldown).toEqual(270549120);
        expect(trnnut.verifyContract(contract_address)).toEqual(true);

    });
});
