window.BENCHMARK_DATA = {
  "lastUpdate": 1744620612252,
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
      }
    ]
  }
}
