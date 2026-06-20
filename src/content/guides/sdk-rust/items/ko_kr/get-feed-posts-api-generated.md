req
tenantId
afterId

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| after_id | String | 아니요 |  |
| limit | i32 | 아니요 |  |
| tags | Vec<String> | 아니요 |  |

## 응답

반환: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_feed_posts_response.rs)

## 예제

[inline-code-attrs-start title = 'get_feed_posts Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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