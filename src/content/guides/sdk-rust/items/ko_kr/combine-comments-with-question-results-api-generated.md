---
## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| question_id | String | 아니오 |  |
| question_ids | Vec<String> | 아니오 |  |
| url_id | String | 아니오 |  |
| start_date | chrono::DateTime<chrono::FixedOffset> | 아니오 |  |
| force_recalculate | bool | 아니오 |  |
| min_value | f64 | 아니오 |  |
| max_value | f64 | 아니오 |  |
| limit | f64 | 아니오 |  |

## 응답

반환: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_question_results_with_comments_response.rs)

## 예제

[inline-code-attrs-start title = 'combine_comments_with_question_results 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: CombineCommentsWithQuestionResultsParams = CombineCommentsWithQuestionResultsParams {
    tenant_id: "acme-corp-tenant".to_string(),
    question_id: None,
    question_ids: Some(vec!["product-satisfaction".to_string(), "support-response".to_string()]),
    url_id: Some("news/article-42".to_string()),
    start_date: Some(chrono::FixedOffset::east(0).ymd(2025, 12, 01).and_hms(08, 00, 00)),
    force_recalculate: Some(true),
    min_value: Some(0.0),
    max_value: Some(1.0),
    limit: Some(250.0),
};
let response: CombineQuestionResultsWithCommentsResponse = combine_comments_with_question_results(&configuration, params).await?;
[inline-code-end]

---