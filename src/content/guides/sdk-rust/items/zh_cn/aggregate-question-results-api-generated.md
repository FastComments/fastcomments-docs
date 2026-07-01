## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| question_id | String | 否 |  |
| question_ids | Vec<String> | 否 |  |
| url_id | String | 否 |  |
| time_bucket | models::AggregateTimeBucket | 否 |  |
| start_date | chrono::DateTime<chrono::FixedOffset> | 否 |  |
| force_recalculate | bool | 否 |  |

## 响应

返回: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_response.rs)

## 示例

[inline-code-attrs-start title = 'aggregate_question_results 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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