## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| direction | String | Nee |  |
| broadcast_id | String | Nee |  |
| sso | String | Nee |  |

## Respons

Retourneert: [`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'post_vote Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let cfg = configuration::Configuration::default();
    let params = PostVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-12345".to_string(),
        direction: Some("up".to_string()),
        broadcast_id: Some("broadcast-987".to_string()),
        sso: None,
    };
    let _response = post_vote(&cfg, params).await?;
    Ok(())
}
[inline-code-end]