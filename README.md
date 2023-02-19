# AHC018

## スコア履歴

|       sum | sum(log) | コメント                                      |
|----------:|---------:|:------------------------------------------|
| 316620396 |  12390.8 | 0-1 bfs (斜め)。dig は 100 固定                 |
| 292483165 |  12316.7 | 0-1 bfs (斜め) * 10。dig は 100 固定            |
| 288956885 |  12358.3 | 0-1 bfs (斜め) * 10。dig を盤面全体平均に対する最適 c にする |

|       sum | sum(log) |     sum i | sum(log i) |   score | comment                                        |
|----------:|---------:|----------:|-----------:|--------:|:-----------------------------------------------|
| 288956885 |  12358.3 | 210114063 |    11983.9 | 714.862 | 0-1 bfs (斜め) * 10。dig を盤面全体平均に対する最適 c (バグ）aにする |
| 272286364 |  12276.0 | 210114063 |    11983.9 | 764.116 | 0-1 bfs (斜め) * 10。dig を盤面全体平均に対する最適 c にする      |
| 262449875 |  12247.4 | 202930274 |    11964.0 | 769.048 | 0-1 bfs (直角) * 100。dig を盤面全体平均に対する最適 c にする     |
| 260769897 |  12246.3 | 200860680 |    11959.7 | 766.745 | 最小全域木 * 100。dig を盤面全体平均に対する最適 c にする            |
