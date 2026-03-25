## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenant_id | String | Oui |  |
| comment_id | String | Oui |  |
| vote_id | String | Oui |  |
| url_id | String | Oui |  |
| broadcast_id | String | Oui |  |
| edit_key | String | Non |  |
| sso | String | Non |  |

## Réponse

Renvoie : [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_vote_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de delete_comment_vote'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteCommentVoteParams = DeleteCommentVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-12345".to_string(),
        vote_id: "vote-67890".to_string(),
        url_id: "news/world/article-2026".to_string(),
        broadcast_id: "broadcast-1".to_string(),
        edit_key: Some("editkey-abc123".to_string()),
        sso: Some("sso-token-xyz".to_string()),
    };
    let response: DeleteCommentVote200Response = delete_comment_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---