## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| question_id | String | いいえ |  |
| question_ids | Vec<String> | いいえ |  |
| url_id | String | いいえ |  |
| time_bucket | models::AggregateTimeBucket | いいえ |  |
| start_date | chrono::DateTime<chrono::FixedOffset> | いいえ |  |
| force_recalculate | bool | いいえ |  |

## 応答

返り値: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_response.rs)

## 例

[inline-code-attrs-start title = 'aggregate_question_results の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = AggregateQuestionResultsParams {
    tenant_id: "acme-corp-tenant".to_string(),
    question_id: Some("question-123".to_string()),
    question_ids: Some(vec!["question-123".to_string(), "question-456".to_string()]),
    url_id: Some("news/article".to_string()),
    time_bucket: Some(models::AggregateTimeBucket::Day),
    start_date: Some(chrono::DateTime::parse_from_rfc3339("2023-01-01T00:00:00+00:00").unwrap()),
    force_recalculate: Some(true),
};

let response = aggregate_question_results(&configuration, params).await?;
[inline-code-end]