---
## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| url_id | String | Non |  |
| user_id | String | Non |  |
| start_date | String | Non |  |
| question_id | String | Non |  |
| question_ids | String | Non |  |
| skip | f64 | Non |  |

## Réponse

Renvoie: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_results_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_question_results'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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