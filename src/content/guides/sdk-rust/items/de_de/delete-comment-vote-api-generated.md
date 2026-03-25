## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| vote_id | String | Ja |  |
| url_id | String | Ja |  |
| broadcast_id | String | Ja |  |
| edit_key | String | Nein |  |
| sso | String | Nein |  |

## Antwort

Gibt zurück: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_vote_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'delete_comment_vote Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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