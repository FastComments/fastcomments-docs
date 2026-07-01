## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| comment_id | String | Sì |  |
| vote_id | String | Sì |  |
| broadcast_id | String | No |  |
| sso | String | No |  |

## Risposta

Restituisce: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio delete_moderation_vote'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = DeleteModerationVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-42".to_string(),
        vote_id: "vote-12345".to_string(),
        broadcast_id: Some("broadcast-987".to_string()),
        sso: None,
    };
    let _response: VoteDeleteResponse = delete_moderation_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]