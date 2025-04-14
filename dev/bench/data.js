window.BENCHMARK_DATA = {
  "lastUpdate": 1744626586258,
  "repoUrl": "https://github.com/contentstech-com/lazycsv",
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
      }
    ]
  }
}