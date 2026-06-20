---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| url_id | String | いいえ |  |
| user_id | String | いいえ |  |
| start_date | String | いいえ |  |
| question_id | String | いいえ |  |
| question_ids | String | いいえ |  |
| skip | f64 | いいえ |  |

## レスポンス

戻り値: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_results_response.rs)

## 例

[inline-code-attrs-start title = 'get_question_results の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetQuestionResultsParams = GetQuestionResultsParams {
    tenant_id: "acme-corp-tenant".to_string(),
    url_id: Some("news/world/2026-election".to_string()),
    user_id: Some("user_12345".to_string()),
    start_date: Some("2026-01-01T00:00:00Z".to_string()),
    question_id: Some("q_987".to_string()),
    question_ids: Some("q_987,q_654".to_string()),
    skip: Some(20.0),
};

let response: GetQuestionResultsResponse = get_question_results(&configuration, params).await?;
[inline-code-end]

---