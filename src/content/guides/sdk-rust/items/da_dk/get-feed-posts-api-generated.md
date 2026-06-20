req
tenantId
afterId

## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| after_id | String | Nej |  |
| limit | i32 | Nej |  |
| tags | Vec<String> | Nej |  |

## Svar

Returnerer: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_response.rs)

## Eksempel

[inline-code-attrs-start title = 'get_feed_posts Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetFeedPostsResponse, Error> {
    let cfg: &configuration::Configuration = &configuration;
    let params: GetFeedPostsParams = GetFeedPostsParams {
        tenant_id: String::from("acme-corp-tenant"),
        after_id: Some(String::from("post_987654321")),
        limit: Some(25),
        tags: Some(vec![String::from("product-updates"), String::from("release")]),
    };
    let response: GetFeedPostsResponse = get_feed_posts(cfg, params).await?;
    Ok(response)
}
[inline-code-end]