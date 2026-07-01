## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| post_id | String | はい |  |
| update_feed_post_params | models::UpdateFeedPostParams | はい |  |
| broadcast_id | String | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

返却: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_post_response.rs)

## 例

[inline-code-attrs-start title = 'update_feed_post_public の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = UpdateFeedPostPublicParams {
    tenant_id: "acme-corp-tenant".into(),
    post_id: "news/article-123".into(),
    update_feed_post_params: models::UpdateFeedPostParams {
        title: Some("Updated Headline".into()),
        content: Some("Revised content of the article with latest information.".into()),
        ..Default::default()
    },
    broadcast_id: Some("broadcast-001".into()),
    sso: Some("sso-token-abc123".into()),
};

let response: CreateFeedPostResponse = update_feed_post_public(&configuration, params).await?;
[inline-code-end]