## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| question_id | String | Não |  |
| question_ids | Vec<String> | Não |  |
| url_id | String | Não |  |
| start_date | String | Não |  |
| force_recalculate | bool | Não |  |
| min_value | f64 | Não |  |
| max_value | f64 | Não |  |
| limit | f64 | Não |  |

## Resposta

Retorna: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_comments_with_question_results_200_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de combine_comments_with_question_results'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<CombineCommentsWithQuestionResults200Response, Error> {
    let params: CombineCommentsWithQuestionResultsParams = CombineCommentsWithQuestionResultsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        question_id: Some("q-2026-product-satisfaction".to_string()),
        question_ids: Some(vec![
            "q-2026-product-satisfaction".to_string(),
            "q-2026-support-rating".to_string(),
        ]),
        url_id: Some("news/product/launch-2026".to_string()),
        start_date: Some("2026-03-01T00:00:00Z".to_string()),
        force_recalculate: Some(true),
        min_value: Some(1.0),
        max_value: Some(5.0),
        limit: Some(100.0),
    };
    let response: CombineCommentsWithQuestionResults200Response =
        combine_comments_with_question_results(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---