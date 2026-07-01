## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| comment_id | String | 是 |  |
| sso | String | 否 |  |

## 响应

返回：[`ModerationApiGetLogsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_logs_response.rs)

## 示例

[inline-code-attrs-start title = '获取日志 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetLogsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-12345".to_string(),
        sso: Some("user@example.com".to_string()),
    };
    let response = get_logs(&configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]

---