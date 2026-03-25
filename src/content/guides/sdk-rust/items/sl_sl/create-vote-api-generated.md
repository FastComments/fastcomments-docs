---
## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| comment_id | String | Da |  |
| direction | String | Da |  |
| user_id | String | Ne |  |
| anon_user_id | String | Ne |  |

## Odgovor

Vrne: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

## Primer

[inline-code-attrs-start title = 'create_vote Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_create_vote() -> Result<(), Error> {
    let params: CreateVoteParams = CreateVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/12345".to_string(),
        direction: "up".to_string(),
        user_id: Some("user-9876".to_string()),
        anon_user_id: Some("anon-01-abcdef".to_string()),
    };

    let response: VoteComment200Response = create_vote(&configuration, params).await?;
    println!("{:?}", response);
    Ok(())
}
[inline-code-end]

---