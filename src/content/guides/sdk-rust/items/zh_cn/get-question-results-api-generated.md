## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| url_id | String | 否 |  |
| user_id | String | 否 |  |
| start_date | String | 否 |  |
| question_id | String | 否 |  |
| question_ids | String | 否 |  |
| skip | f64 | 否 |  |

## 响应

返回: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_results_200_response.rs)

## 示例

[inline-code-attrs-start title = 'get_question_results 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetQuestionResults200Response, Error> {
    let params: GetQuestionResultsParams = GetQuestionResultsParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        url_id: Some("news/local/2026/03/25".to_owned()),
        user_id: Some("user_12345".to_owned()),
        start_date: Some("2026-01-01T00:00:00Z".to_owned()),
        question_id: Some("q_789".to_owned()),
        question_ids: Some("q_789,q_790".to_owned()),
        skip: Some(10.0),
    };
    let response: GetQuestionResults200Response = get_question_results(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---