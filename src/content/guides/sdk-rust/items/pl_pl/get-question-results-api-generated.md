---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| url_id | String | Nie |  |
| user_id | String | Nie |  |
| start_date | String | Nie |  |
| question_id | String | Nie |  |
| question_ids | String | Nie |  |
| skip | f64 | Nie |  |

## Odpowiedź

Zwraca: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_results_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_question_results'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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