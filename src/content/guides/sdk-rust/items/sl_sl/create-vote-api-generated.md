## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| comment_id | String | Da |  |
| direction | String | Da |  |
| user_id | String | Ne |  |
| anon_user_id | String | Ne |  |

## Odgovor

Vrne: [`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## Primer

[inline-code-attrs-start title = 'create_vote Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<VoteResponse, Error> {
    let params: CreateVoteParams = CreateVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/12345/comment-9876".to_string(),
        direction: "up".to_string(),
        user_id: Some("user-42".to_string()),
        anon_user_id: None,
    };
    let vote_response: VoteResponse = create_vote(&configuration, params).await?;
    Ok(vote_response)
}
[inline-code-end]

---