## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| comment_id | String | Da |  |
| vote_id | String | Da |  |
| url_id | String | Da |  |
| broadcast_id | String | Da |  |
| edit_key | String | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_vote_200_response.rs)

## Primjer

[inline-code-attrs-start title = 'delete_comment_vote Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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