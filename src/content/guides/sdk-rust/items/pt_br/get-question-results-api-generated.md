## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenant_id | String | Sim |  |
| url_id | String | Não |  |
| user_id | String | Não |  |
| start_date | String | Não |  |
| question_id | String | Não |  |
| question_ids | String | Não |  |
| skip | f64 | Não |  |

## Resposta

Retorna: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_question_results_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_question_results'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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