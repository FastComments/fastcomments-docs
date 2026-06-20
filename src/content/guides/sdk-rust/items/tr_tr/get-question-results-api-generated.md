## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| url_id | String | Hayır |  |
| user_id | String | Hayır |  |
| start_date | String | Hayır |  |
| question_id | String | Hayır |  |
| question_ids | String | Hayır |  |
| skip | f64 | Hayır |  |

## Yanıt

Döndürür: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_results_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_question_results Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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