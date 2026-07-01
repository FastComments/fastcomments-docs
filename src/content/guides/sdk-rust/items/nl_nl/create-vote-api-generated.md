## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| direction | String | Ja |  |
| user_id | String | Nee |  |
| anon_user_id | String | Nee |  |

## Respons

Retourneert: [`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'create_vote Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn submit_vote() -> Result<(), Error> {
    let params = CreateVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/12345".to_string(),
        direction: "up".to_string(),
        user_id: Some("user-42".to_string()),
        anon_user_id: Some("anon-99".to_string()),
    };
    let _response = create_vote(&config, params).await?;
    Ok(())
}
[inline-code-end]