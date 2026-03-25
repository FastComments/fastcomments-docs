---
## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |

## Respons

Retourneert: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'delete_question_result Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_question_result() -> Result<(), Error> {
    let params: DeleteQuestionResultParams = DeleteQuestionResultParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "qres-news/article-2026-03-25-9a8b7c".to_string(),
    };
    let response: FlagCommentPublic200Response = delete_question_result(&configuration, params).await?;
    let _response = response;
    Ok(())
}
[inline-code-end]

---