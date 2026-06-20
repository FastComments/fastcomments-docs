## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| create_user_badge_params | models::CreateUserBadgeParams | 是 |  |

## 响应

返回: [`ApiCreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_create_user_badge_response.rs)

## 示例

[inline-code-attrs-start title = 'create_user_badge 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateUserBadgeParams = CreateUserBadgeParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_user_badge_params: models::CreateUserBadgeParams {
            user_id: "user-7890".to_string(),
            badge_key: "top-commenter".to_string(),
            title: "Top Commenter".to_string(),
            description: Some("Consistently provided insightful comments".to_string()),
            image_url: Some("https://assets.news.example.com/badges/top-commenter.png".to_string()),
            is_visible: Some(true),
            expires_at: None,
        },
    };

    let response: ApiCreateUserBadgeResponse = create_user_badge(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---