## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| comment_id | String | Da |  |
| vote_id | String | Da |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## Primjer

[inline-code-attrs-start title = 'Primjer za delete_moderation_vote'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteModerationVoteParams = DeleteModerationVoteParams {
        comment_id: "news/article-2026-06-19-12345".to_string(),
        vote_id: "vote-9a7c3b1d".to_string(),
        sso: Some("user-9876@acme-corp".to_string()),
    };
    let response: VoteDeleteResponse = delete_moderation_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---