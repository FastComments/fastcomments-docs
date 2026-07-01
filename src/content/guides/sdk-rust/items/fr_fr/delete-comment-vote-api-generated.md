## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| comment_id | String | Oui |  |
| vote_id | String | Oui |  |
| url_id | String | Oui |  |
| broadcast_id | String | Oui |  |
| edit_key | String | Non |  |
| sso | String | Non |  |

## Réponse

Retourne : [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## Exemple

[inline-code-attrs-start title = 'delete_comment_vote Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let params = DeleteCommentVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        vote_id: "vote-67890".to_string(),
        url_id: "news/article".to_string(),
        broadcast_id: "broadcast-abc".to_string(),
        edit_key: Some("edit-key-xyz".to_string()),
        sso: Some("sso-token-123".to_string()),
    };
    let _response: VoteDeleteResponse = delete_comment_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---