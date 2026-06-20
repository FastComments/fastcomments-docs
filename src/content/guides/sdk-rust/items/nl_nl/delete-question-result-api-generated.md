## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |

## Antwoord

Retourneert: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'delete_question_result Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete() -> Result<(), Error> {
    let tenant_id: String = "acme-corp-tenant".to_string();
    let id: String = "news/article-12345/question-67890".to_string();

    let params = DeleteQuestionResultParams {
        tenant_id,
        id,
    };

    let response: ApiEmptyResponse = delete_question_result(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---