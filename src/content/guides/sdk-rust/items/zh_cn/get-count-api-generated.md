## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| text_search | String | 否 |  |
| by_ip_from_comment | String | 否 |  |
| filter | String | 否 |  |
| search_filters | String | 否 |  |
| demo | bool | 否 |  |
| sso | String | 否 |  |

## 响应

返回: [`ModerationApiCountCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_count_comments_response.rs)

## 示例

[inline-code-attrs-start title = 'get_count 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        text_search: Some("breaking news".to_string()),
        by_ip_from_comment: Some("192.168.1.1".to_string()),
        filter: Some("status:approved".to_string()),
        search_filters: Some("author:john".to_string()),
        demo: Some(false),
        sso: Some("sso-token-123".to_string()),
    };
    let _response = get_count(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---