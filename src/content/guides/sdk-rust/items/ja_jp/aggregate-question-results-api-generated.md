## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| question_id | String | いいえ |  |
| question_ids | Vec<String> | いいえ |  |
| url_id | String | いいえ |  |
| time_bucket | models::AggregateTimeBucket | いいえ |  |
| start_date | String | いいえ |  |
| force_recalculate | bool | いいえ |  |

## レスポンス

戻り値: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_200_response.rs)

---