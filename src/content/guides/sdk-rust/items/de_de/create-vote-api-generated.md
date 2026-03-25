## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| direction | String | Ja |  |
| user_id | String | Nein |  |
| anon_user_id | String | Nein |  |

## Antwort

Gibt zurück: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

## Beispiel

[inline-code-attrs-start title = 'create_vote Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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