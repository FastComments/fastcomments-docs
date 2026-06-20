## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| update_question_result_body | models::UpdateQuestionResultBody | Ja |  |

## Respons

Retourneert: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'update_question_result Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_update_question_result() -> Result<(), Error> {
    let body: models::UpdateQuestionResultBody = models::UpdateQuestionResultBody {
        answered: Some(true),
        confidence: Some(0.92),
        responder: Some("editor-zoe".to_string()),
        notes: Some("Validated against article sources".to_string()),
    };
    let params: UpdateQuestionResultParams = UpdateQuestionResultParams {
        tenant_id: "acme-news-tenant".to_string(),
        id: "news/article/5621/question/12".to_string(),
        update_question_result_body: body,
    };
    let _resp: ApiEmptyResponse = update_question_result(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---