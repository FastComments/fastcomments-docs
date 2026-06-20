## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| question_id | String | 아니오 |  |
| question_ids | Vec<String> | 아니오 |  |
| url_id | String | 아니오 |  |
| time_bucket | models::AggregateTimeBucket | 아니오 |  |
| start_date | chrono::DateTime<chrono::FixedOffset> | 아니오 |  |
| force_recalculate | bool | 아니오 |  |

## 응답

반환: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_question_results_response.rs)

## 예제

[inline-code-attrs-start title = 'aggregate_question_results 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<AggregateQuestionResultsResponse, Error> {
    let params: AggregateQuestionResultsParams = AggregateQuestionResultsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        question_id: Some("q-12345".to_string()),
        question_ids: Some(vec!["q-12345".to_string(), "q-67890".to_string()]),
        url_id: Some("news/article/2026/06/breaking".to_string()),
        time_bucket: Some(models::AggregateTimeBucket::Daily),
        start_date: Some(chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00+00:00").unwrap()),
        force_recalculate: Some(true),
    };
    let response: AggregateQuestionResultsResponse = aggregate_question_results(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---