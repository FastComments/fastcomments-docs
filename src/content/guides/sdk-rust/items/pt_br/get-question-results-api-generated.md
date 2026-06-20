## Parâmetros

| Name | Type | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| url_id | String | Não |  |
| user_id | String | Não |  |
| start_date | String | Não |  |
| question_id | String | Não |  |
| question_ids | String | Não |  |
| skip | f64 | Não |  |

## Resposta

Retorna: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_results_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_question_results'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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