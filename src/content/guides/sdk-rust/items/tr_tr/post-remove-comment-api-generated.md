## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| comment_id | String | Evet |  |
| broadcast_id | String | Hayır |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/post_remove_comment_api_response.rs)

## Örnek

[inline-code-attrs-start title = 'post_remove_comment Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn remove_comment_example(config: &configuration::Configuration) -> Result<(), Error> {
    let params = PostRemoveCommentParams {
        tenant_id: "acme-corp".into(),
        comment_id: "news/article/42".into(),
        broadcast_id: Some("live-event-99".into()),
        sso: Some("sso-user-abc".into()),
    };
    let _response = post_remove_comment(config, params).await?;
    Ok(())
}
[inline-code-end]