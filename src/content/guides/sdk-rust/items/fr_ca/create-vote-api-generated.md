## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| comment_id | String | Oui |  |
| direction | String | Oui |  |
| user_id | String | Non |  |
| anon_user_id | String | Non |  |

## Réponse

Renvoie: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de create_vote'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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