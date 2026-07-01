## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| create_feed_post_params | models::CreateFeedPostParams | 是 |  |
| broadcast_id | String | 否 |  |
| is_live | bool | 否 |  |
| do_spam_check | bool | 否 |  |
| skip_dup_check | bool | 否 |  |

## 回應

返回：[`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_feed_posts_response.rs)

## 範例

[inline-code-attrs-start title = 'create_feed_post 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = CreateFeedPostParams {
        tenant_id: "acme-corp-tenant".into(),
        create_feed_post_params: models::CreateFeedPostParams {
            text: "Launching new features".into(),
            media: vec![],
        },
        broadcast_id: Some("broadcast-2023-09".into()),
        is_live: Some(true),
        do_spam_check: Some(true),
        skip_dup_check: Some(false),
    };
    let _response = create_feed_post(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---