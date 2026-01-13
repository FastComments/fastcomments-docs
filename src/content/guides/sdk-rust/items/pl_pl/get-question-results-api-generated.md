## Parametry

| Name | Type | Required | Description |
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
async fn example_get_question_results() -> Result<(), Error> {
    let params = GetQuestionResultsParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: Some(String::from("news/article/2026/01/12/breaking")),
        user_id: Some(String::from("user-98765")),
        start_date: Some(String::from("2025-12-01")),
        question_id: Some(String::from("q-42")),
        question_ids: Some(String::from("q-42,q-43")),
        skip: Some(10.0),
    };
    let results: GetQuestionResults200Response = get_question_results(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---