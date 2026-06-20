## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Nej |  |
| user_id | String | Nej |  |
| start_date | String | Nej |  |
| question_id | String | Nej |  |
| question_ids | String | Nej |  |
| skip | f64 | Nej |  |

## Svar

Returnerer: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_results_response.rs)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på get_question_results'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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