## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |

## Respons

Retourneert: [`ApiGetCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_comment_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld get_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_comment() -> Result<(), Error> {
    let params = GetCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-12345".to_string(),
        include_deleted: Some(false),
    };

    let _response: ApiGetCommentResponse = get_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]