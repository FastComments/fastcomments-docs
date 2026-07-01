## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| comment_id | String | はい |  |
| include_email | bool | いいえ |  |
| include_ip | bool | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`ModerationApiCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_comment_response.rs)

## 例

[inline-code-attrs-start title = 'get_moderation_comment 例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetModerationCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-6789".to_string(),
        include_email: Some(true),
        include_ip: Some(true),
        sso: Some("sso-user-42".to_string()),
    };
    let _response = get_moderation_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]