## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| question_id | String | Nee |  |
| question_ids | Vec<String> | Nee |  |
| url_id | String | Nee |  |
| start_date | String | Nee |  |
| force_recalculate | bool | Nee |  |
| min_value | f64 | Nee |  |
| max_value | f64 | Nee |  |
| limit | f64 | Nee |  |

## Antwoord

Retourneert: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/combine_comments_with_question_results_200_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'combine_comments_with_question_results Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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