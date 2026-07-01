## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_user_badge_params | models::CreateUserBadgeParams | Yes |  |

## 响应

返回: [`ApiCreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_create_user_badge_response.rs)

## 示例

[inline-code-attrs-start title = 'create_user_badge 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = CreateUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_user_badge_params: models::CreateUserBadgeParams {
            badge_type: "premium".to_string(),
            user_id: "user-123".to_string(),
            description: Some("Top contributor".to_string()),
            expires_at: None,
        },
    };
    let _response = create_user_badge(&configuration, params).await?;
    Ok(())
}
[inline-code-end]