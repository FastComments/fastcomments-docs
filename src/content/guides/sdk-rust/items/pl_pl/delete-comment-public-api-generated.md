## Parameters

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenant_id | String | Tak |  |
| comment_id | String | Tak |  |
| broadcast_id | String | Tak |  |
| edit_key | String | Nie |  |
| sso | String | Nie |  |

## Response

Zwraca: [`PublicApiDeleteCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_api_delete_comment_response.rs)

## Example

[inline-code-attrs-start title = 'delete_comment_public Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete() -> Result<(), Error> {
    let params = DeleteCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-12345".to_string(),
        broadcast_id: "news/article-6789".to_string(),
        edit_key: Some("edit-abc123".to_string()),
        sso: Some("sso-token-xyz".to_string()),
    };
    let response = delete_comment_public(&configuration, params).await?;
    let _deleted: PublicApiDeleteCommentResponse = response;
    Ok(())
}
[inline-code-end]