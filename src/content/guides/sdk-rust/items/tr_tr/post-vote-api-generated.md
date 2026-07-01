## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| comment_id | String | Evet |  |
| direction | String | Hayır |  |
| broadcast_id | String | Hayır |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## Örnek

[inline-code-attrs-start title = 'post_vote Örnek'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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