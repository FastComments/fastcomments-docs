## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## 响应

返回：[`ApiGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_response.rs)

## 示例

[inline-code-attrs-start title = 'get_user_badge 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_badge() -> Result<(), Error> {
    let params = GetUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "badge-42".to_string(),
    };
    let _response: ApiGetUserBadgeResponse = get_user_badge(&configuration, params).await?;
    Ok(())
}
[inline-code-end]