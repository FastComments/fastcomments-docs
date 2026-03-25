## Parametreler

| Name | Type | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| url_id | String | Hayır |  |
| user_id | String | Hayır |  |
| start_date | String | Hayır |  |
| question_id | String | Hayır |  |
| question_ids | String | Hayır |  |
| skip | f64 | Hayır |  |

## Yanıt

Döndürür: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_results_200_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_question_results Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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