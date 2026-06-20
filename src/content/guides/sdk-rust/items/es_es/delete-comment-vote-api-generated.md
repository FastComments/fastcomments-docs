## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenant_id | String | Sí |  |
| comment_id | String | Sí |  |
| vote_id | String | Sí |  |
| url_id | String | Sí |  |
| broadcast_id | String | Sí |  |
| edit_key | String | No |  |
| sso | String | No |  |

## Respuesta

Devuelve: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de delete_comment_vote'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_delete_vote() -> Result<VoteDeleteResponse, Error> {
    let params: DeleteCommentVoteParams = DeleteCommentVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-8f3a2b9e".to_string(),
        vote_id: "vote-7d124f".to_string(),
        url_id: "news/politics/2026-election".to_string(),
        broadcast_id: "web-1234".to_string(),
        edit_key: Some("edit-abc123".to_string()),
        sso: Some("sso-token-xyz".to_string()),
    };

    let response: VoteDeleteResponse = delete_comment_vote(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---