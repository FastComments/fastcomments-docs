## 参数

| 名称 | 类型 | 必需 | 说明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 否 |  |
| user_id | String | 否 |  |
| start_date | String | 否 |  |
| question_id | String | 否 |  |
| question_ids | String | 否 |  |
| skip | f64 | 否 |  |

## 响应

返回: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_results_response.rs)

## 示例

[inline-code-attrs-start title = 'get_question_results 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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