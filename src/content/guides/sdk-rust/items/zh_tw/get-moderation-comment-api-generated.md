## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| include_email | bool | 否 |  |
| include_ip | bool | 否 |  |
| sso | String | 否 |  |

## 回應

返回：[`ModerationApiCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_comment_response.rs)

## 範例

[inline-code-attrs-start title = 'get_moderation_comment 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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