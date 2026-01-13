## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| comment_id | String | Da |  |
| direction | String | Da |  |
| user_id | String | Ne |  |
| anon_user_id | String | Ne |  |

## Odgovor

Vraća: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

## Primjer

[inline-code-attrs-start title = 'create_vote Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_vote() -> Result<VoteComment200Response, Error> {
    let params: CreateVoteParams = CreateVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/2026/01/12/local-election-12345".to_string(),
        direction: "up".to_string(),
        user_id: Some("user_9876".to_string()),
        anon_user_id: None,
    };
    let response: VoteComment200Response = create_vote(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---