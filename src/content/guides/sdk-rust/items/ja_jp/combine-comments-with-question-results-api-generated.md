## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| question_id | String | いいえ |  |
| question_ids | Vec<String> | いいえ |  |
| url_id | String | いいえ |  |
| start_date | String | いいえ |  |
| force_recalculate | bool | いいえ |  |
| min_value | f64 | いいえ |  |
| max_value | f64 | いいえ |  |
| limit | f64 | いいえ |  |

## レスポンス

戻り値: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_comments_with_question_results_200_response.rs)

---