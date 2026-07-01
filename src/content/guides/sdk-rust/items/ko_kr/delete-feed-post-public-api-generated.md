## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenant_id | String | 예 |  |
| post_id | String | 예 |  |
| broadcast_id | String | 아니오 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_feed_post_public_response.rs)

## 예제

[inline-code-attrs-start title = 'delete_feed_post_public 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = DeleteFeedPostPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_id: "news/article-123".to_string(),
        broadcast_id: Some("broadcast-456".to_string()),
        sso: Some("sso-token-789".to_string()),
    };
    let _response: DeleteFeedPostPublicResponse = delete_feed_post_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]