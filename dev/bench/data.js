window.BENCHMARK_DATA = {
  "lastUpdate": 1761712165012,
  "repoUrl": "https://github.com/contentstech-com/crates",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "me@xiniha.dev",
            "name": "Iha Shin",
            "username": "XiNiHa"
          },
          "committer": {
            "email": "simnalamburt@gmail.com",
            "name": "Jihyeon Kim",
            "username": "simnalamburt"
          },
          "distinct": false,
          "id": "186d64d67b7a86bbd9200deaccce9b9a550aa1ca",
          "message": "bench: add different types of inputs",
          "timestamp": "2025-04-14T17:27:24+09:00",
          "tree_id": "c33401ed91550818b6202f219f4220af20aee16e",
          "url": "https://github.com/contentstech-com/lazycsv/commit/186d64d67b7a86bbd9200deaccce9b9a550aa1ca"
        },
        "date": 1744620611933,
        "tool": "cargo",
        "benches": [
          {
            "name": "No quotes/lazy_csv",
            "value": 132108489,
            "range": "± 493302",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows)",
            "value": 146425101,
            "range": "± 2279260",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (raw)",
            "value": 64086555,
            "range": "± 788617",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows, raw)",
            "value": 67102740,
            "range": "± 573338",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/csv",
            "value": 159588883,
            "range": "± 381194",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv",
            "value": 645422015,
            "range": "± 1863146",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows)",
            "value": 652199560,
            "range": "± 825393",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (raw)",
            "value": 179580621,
            "range": "± 1861712",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows, raw)",
            "value": 176799574,
            "range": "± 233025",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/csv",
            "value": 278472219,
            "range": "± 582756",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv",
            "value": 419511459,
            "range": "± 2002018",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows)",
            "value": 425327381,
            "range": "± 491491",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (raw)",
            "value": 133967706,
            "range": "± 1774198",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows, raw)",
            "value": 134826447,
            "range": "± 3520754",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/csv",
            "value": 241667788,
            "range": "± 1189856",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "simnalamburt@gmail.com",
            "name": "Jihyeon Kim",
            "username": "simnalamburt"
          },
          "committer": {
            "email": "simnalamburt@gmail.com",
            "name": "Jihyeon Kim",
            "username": "simnalamburt"
          },
          "distinct": false,
          "id": "ec842c71ac2c0aa7e23cd540276a5b8bcdc90491",
          "message": "Add sufficient explanation regarding lazycsv’s performance characteristics",
          "timestamp": "2025-04-14T19:02:02+09:00",
          "tree_id": "32d14f0f09881a032d15da2bf5af5262cba161b6",
          "url": "https://github.com/contentstech-com/lazycsv/commit/ec842c71ac2c0aa7e23cd540276a5b8bcdc90491"
        },
        "date": 1744626585404,
        "tool": "cargo",
        "benches": [
          {
            "name": "No quotes/lazy_csv",
            "value": 138143068,
            "range": "± 912247",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows)",
            "value": 144932071,
            "range": "± 3757378",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (raw)",
            "value": 66128026,
            "range": "± 219783",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows, raw)",
            "value": 69764117,
            "range": "± 1234050",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/csv",
            "value": 160449393,
            "range": "± 860526",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv",
            "value": 645081512,
            "range": "± 728664",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows)",
            "value": 649816960,
            "range": "± 6142718",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (raw)",
            "value": 178931024,
            "range": "± 1600594",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows, raw)",
            "value": 178365875,
            "range": "± 3265158",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/csv",
            "value": 277680138,
            "range": "± 3397421",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv",
            "value": 423697092,
            "range": "± 1848374",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows)",
            "value": 423374350,
            "range": "± 414190",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (raw)",
            "value": 134716166,
            "range": "± 753100",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows, raw)",
            "value": 135096193,
            "range": "± 290550",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/csv",
            "value": 241867280,
            "range": "± 3912906",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "simnalamburt@gmail.com",
            "name": "Jihyeon Kim",
            "username": "simnalamburt"
          },
          "committer": {
            "email": "simnalamburt@gmail.com",
            "name": "Jihyeon Kim",
            "username": "simnalamburt"
          },
          "distinct": true,
          "id": "655f1dbd1e4b569fa802fc2c158f52d8cff382ae",
          "message": "v0.2.1",
          "timestamp": "2025-04-15T14:53:53+09:00",
          "tree_id": "12ec1c34c378de4cbe3117be0b8ac8e4f5bfc946",
          "url": "https://github.com/contentstech-com/lazycsv/commit/655f1dbd1e4b569fa802fc2c158f52d8cff382ae"
        },
        "date": 1744696842786,
        "tool": "cargo",
        "benches": [
          {
            "name": "No quotes/lazy_csv",
            "value": 132090482,
            "range": "± 2568608",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows)",
            "value": 144554024,
            "range": "± 5596697",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (raw)",
            "value": 66046343,
            "range": "± 1394605",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows, raw)",
            "value": 69619366,
            "range": "± 899677",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/csv",
            "value": 161338519,
            "range": "± 1839421",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv",
            "value": 642249156,
            "range": "± 5494578",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows)",
            "value": 644696912,
            "range": "± 7944076",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (raw)",
            "value": 179546352,
            "range": "± 1727178",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows, raw)",
            "value": 178453237,
            "range": "± 837227",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/csv",
            "value": 277348079,
            "range": "± 335767",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv",
            "value": 419374686,
            "range": "± 3483872",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows)",
            "value": 420931261,
            "range": "± 4108596",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (raw)",
            "value": 134544302,
            "range": "± 1226911",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows, raw)",
            "value": 135134271,
            "range": "± 1078878",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/csv",
            "value": 241211485,
            "range": "± 561787",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "simnalamburt@gmail.com",
            "name": "Jihyeon Kim",
            "username": "simnalamburt"
          },
          "committer": {
            "email": "simnalamburt@gmail.com",
            "name": "Jihyeon Kim",
            "username": "simnalamburt"
          },
          "distinct": false,
          "id": "761207ad0f64a8f89841b9f7213fd5e25a71e24e",
          "message": ".github: Pin cargo-criterion version",
          "timestamp": "2025-04-18T18:31:47+09:00",
          "tree_id": "e58e386ae6e3d69dfe006e9c1c5c471eb55408d1",
          "url": "https://github.com/contentstech-com/crates/commit/761207ad0f64a8f89841b9f7213fd5e25a71e24e"
        },
        "date": 1744970514486,
        "tool": "cargo",
        "benches": [
          {
            "name": "No quotes/lazy_csv",
            "value": 130774167,
            "range": "± 505811",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows)",
            "value": 146801463,
            "range": "± 1362309",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (raw)",
            "value": 64525862,
            "range": "± 121551",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows, raw)",
            "value": 67210355,
            "range": "± 375812",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/csv",
            "value": 159813763,
            "range": "± 539078",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv",
            "value": 648669060,
            "range": "± 4416132",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows)",
            "value": 648714776,
            "range": "± 1111283",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (raw)",
            "value": 179520123,
            "range": "± 271540",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows, raw)",
            "value": 176921577,
            "range": "± 753501",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/csv",
            "value": 278779339,
            "range": "± 1311615",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv",
            "value": 423307403,
            "range": "± 1249636",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows)",
            "value": 425250763,
            "range": "± 2200424",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (raw)",
            "value": 135783425,
            "range": "± 647183",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows, raw)",
            "value": 134796561,
            "range": "± 559992",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/csv",
            "value": 241766268,
            "range": "± 481401",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "simnalamburt@gmail.com",
            "name": "Jihyeon Kim",
            "username": "simnalamburt"
          },
          "committer": {
            "email": "simnalamburt@gmail.com",
            "name": "Jihyeon Kim",
            "username": "simnalamburt"
          },
          "distinct": true,
          "id": "abbb875fedd0e7c873e0a9cdbf4bb2b9c9334d85",
          "message": "Create a root-level README.md",
          "timestamp": "2025-04-18T19:06:23+09:00",
          "tree_id": "30bec10e8cf5f399a64c25b60ead7cc601a6fe77",
          "url": "https://github.com/contentstech-com/crates/commit/abbb875fedd0e7c873e0a9cdbf4bb2b9c9334d85"
        },
        "date": 1744971144699,
        "tool": "cargo",
        "benches": [
          {
            "name": "No quotes/lazy_csv",
            "value": 130445481,
            "range": "± 255194",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows)",
            "value": 145705740,
            "range": "± 1989908",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (raw)",
            "value": 66619128,
            "range": "± 234997",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows, raw)",
            "value": 66941610,
            "range": "± 142542",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/csv",
            "value": 160394823,
            "range": "± 525607",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv",
            "value": 652348959,
            "range": "± 1384459",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows)",
            "value": 645492166,
            "range": "± 1249060",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (raw)",
            "value": 180465598,
            "range": "± 3682754",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows, raw)",
            "value": 177434265,
            "range": "± 177303",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/csv",
            "value": 276970010,
            "range": "± 278170",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv",
            "value": 423497050,
            "range": "± 1640382",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows)",
            "value": 425725889,
            "range": "± 1492327",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (raw)",
            "value": 134623270,
            "range": "± 2997899",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows, raw)",
            "value": 134826993,
            "range": "± 3150258",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/csv",
            "value": 241313654,
            "range": "± 317302",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "simnalamburt@gmail.com",
            "name": "Jihyeon Kim",
            "username": "simnalamburt"
          },
          "committer": {
            "email": "simnalamburt@gmail.com",
            "name": "Jihyeon Kim",
            "username": "simnalamburt"
          },
          "distinct": true,
          "id": "5f67ee85c1c4512ffb3eb08a0544a41f5936d60f",
          "message": "Rename packages -> crates",
          "timestamp": "2025-04-18T19:36:06+09:00",
          "tree_id": "a8f71cd1870c9519bddc4906bb5250ecfc54b261",
          "url": "https://github.com/contentstech-com/crates/commit/5f67ee85c1c4512ffb3eb08a0544a41f5936d60f"
        },
        "date": 1744972933051,
        "tool": "cargo",
        "benches": [
          {
            "name": "No quotes/lazy_csv",
            "value": 131160177,
            "range": "± 1546042",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows)",
            "value": 145474020,
            "range": "± 733991",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (raw)",
            "value": 66194114,
            "range": "± 174878",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows, raw)",
            "value": 70697282,
            "range": "± 192093",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/csv",
            "value": 159793305,
            "range": "± 638278",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv",
            "value": 657701191,
            "range": "± 1019128",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows)",
            "value": 648846005,
            "range": "± 963857",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (raw)",
            "value": 178703909,
            "range": "± 1478177",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows, raw)",
            "value": 177633032,
            "range": "± 464610",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/csv",
            "value": 277581594,
            "range": "± 591108",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv",
            "value": 423210172,
            "range": "± 2469426",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows)",
            "value": 422438659,
            "range": "± 327741",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (raw)",
            "value": 134759214,
            "range": "± 800790",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows, raw)",
            "value": 138204561,
            "range": "± 1109256",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/csv",
            "value": 241881517,
            "range": "± 1006689",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@xiniha.dev",
            "name": "Iha Shin",
            "username": "XiNiHa"
          },
          "committer": {
            "email": "me@xiniha.dev",
            "name": "Iha Shin",
            "username": "XiNiHa"
          },
          "distinct": true,
          "id": "67322bd49efbfd3a9aaae001d1013396bc925964",
          "message": "lazycsv: Fix tests without features enabled",
          "timestamp": "2025-04-28T20:36:38+09:00",
          "tree_id": "4a6111a7ec8653ab14f5765aa831e0eba023139c",
          "url": "https://github.com/contentstech-com/crates/commit/67322bd49efbfd3a9aaae001d1013396bc925964"
        },
        "date": 1745840614674,
        "tool": "cargo",
        "benches": [
          {
            "name": "No quotes/lazy_csv",
            "value": 136397782,
            "range": "± 605928",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows)",
            "value": 149701921,
            "range": "± 776318",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (raw)",
            "value": 65902215,
            "range": "± 321384",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows, raw)",
            "value": 68778390,
            "range": "± 367680",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/csv",
            "value": 158687046,
            "range": "± 870792",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv",
            "value": 669653497,
            "range": "± 2532305",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows)",
            "value": 664970020,
            "range": "± 1262359",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (raw)",
            "value": 180306166,
            "range": "± 1223740",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows, raw)",
            "value": 177581423,
            "range": "± 2275446",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/csv",
            "value": 277772938,
            "range": "± 2231630",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv",
            "value": 437721109,
            "range": "± 985203",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows)",
            "value": 440510916,
            "range": "± 1162815",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (raw)",
            "value": 135023520,
            "range": "± 316242",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows, raw)",
            "value": 136439369,
            "range": "± 309333",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/csv",
            "value": 239733948,
            "range": "± 923519",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "simnalamburt@gmail.com",
            "name": "Jihyeon Kim",
            "username": "simnalamburt"
          },
          "committer": {
            "email": "simnalamburt@gmail.com",
            "name": "Jihyeon Kim",
            "username": "simnalamburt"
          },
          "distinct": false,
          "id": "4e6cc25bf6c8030676d12eb537486974d3803e5b",
          "message": "lazycsv: Fix example codes",
          "timestamp": "2025-10-28T20:49:08+09:00",
          "tree_id": "3cadbe56e60f6638449283b2f4970f300c44339d",
          "url": "https://github.com/contentstech-com/crates/commit/4e6cc25bf6c8030676d12eb537486974d3803e5b"
        },
        "date": 1761653959014,
        "tool": "cargo",
        "benches": [
          {
            "name": "No quotes/lazy_csv",
            "value": 133697900,
            "range": "± 886123",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows)",
            "value": 150013411,
            "range": "± 138059",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (raw)",
            "value": 66577637,
            "range": "± 100166",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows, raw)",
            "value": 69469336,
            "range": "± 307600",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/csv",
            "value": 161266822,
            "range": "± 618340",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv",
            "value": 642092836,
            "range": "± 1314529",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows)",
            "value": 651708126,
            "range": "± 1343806",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (raw)",
            "value": 180039545,
            "range": "± 129389",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows, raw)",
            "value": 177074387,
            "range": "± 193486",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/csv",
            "value": 277504185,
            "range": "± 571542",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv",
            "value": 418991619,
            "range": "± 404614",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows)",
            "value": 426212085,
            "range": "± 162880",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (raw)",
            "value": 134659710,
            "range": "± 159862",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows, raw)",
            "value": 136349103,
            "range": "± 140841",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/csv",
            "value": 242889278,
            "range": "± 1706376",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "simnalamburt@gmail.com",
            "name": "Jihyeon Kim",
            "username": "simnalamburt"
          },
          "committer": {
            "email": "simnalamburt@gmail.com",
            "name": "Jihyeon Kim",
            "username": "simnalamburt"
          },
          "distinct": true,
          "id": "91045821c91b63a10254929c51361ccecd6583c5",
          "message": "lazycsv: v0.3.1",
          "timestamp": "2025-10-29T13:22:42+09:00",
          "tree_id": "d89a769793c94f2d8245c61fa3e059d0e3d4062c",
          "url": "https://github.com/contentstech-com/crates/commit/91045821c91b63a10254929c51361ccecd6583c5"
        },
        "date": 1761712164707,
        "tool": "cargo",
        "benches": [
          {
            "name": "No quotes/lazy_csv",
            "value": 135423923,
            "range": "± 1273968",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows)",
            "value": 149951772,
            "range": "± 277094",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (raw)",
            "value": 66575923,
            "range": "± 489002",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/lazy_csv (into_rows, raw)",
            "value": 70507582,
            "range": "± 334153",
            "unit": "ns/iter"
          },
          {
            "name": "No quotes/csv",
            "value": 159030575,
            "range": "± 285418",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv",
            "value": 643141977,
            "range": "± 1490689",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows)",
            "value": 658522320,
            "range": "± 1210278",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (raw)",
            "value": 180406750,
            "range": "± 436259",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/lazy_csv (into_rows, raw)",
            "value": 181374255,
            "range": "± 353649",
            "unit": "ns/iter"
          },
          {
            "name": "Always quoted/csv",
            "value": 279074548,
            "range": "± 1713718",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv",
            "value": 421599449,
            "range": "± 918412",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows)",
            "value": 430170423,
            "range": "± 1144914",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (raw)",
            "value": 134970537,
            "range": "± 276448",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/lazy_csv (into_rows, raw)",
            "value": 138686239,
            "range": "± 536704",
            "unit": "ns/iter"
          },
          {
            "name": "Randomly quoted/csv",
            "value": 241230458,
            "range": "± 472770",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}