## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Так |  |
| url_id | String | Ні |  |
| user_id | String | Ні |  |
| start_date | String | Ні |  |
| question_id | String | Ні |  |
| question_ids | String | Ні |  |
| skip | f64 | Ні |  |

## Відповідь

Повертає: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_results_200_response.rs)

## Приклад

[inline-code-attrs-start title = 'get_question_results Приклад'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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