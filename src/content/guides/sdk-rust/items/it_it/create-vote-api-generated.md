---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| comment_id | String | Sì |  |
| direction | String | Sì |  |
| user_id | String | No |  |
| anon_user_id | String | No |  |

## Risposta

Restituisce: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di create_vote'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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