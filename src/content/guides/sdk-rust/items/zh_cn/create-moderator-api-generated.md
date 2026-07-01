## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| create_moderator_body | models::CreateModeratorBody | Yes |  |

## 响应

返回: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_moderator_response.rs)

## 示例

[inline-code-attrs-start title = 'create_moderator 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = CreateModeratorParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_moderator_body: models::CreateModeratorBody {
            email: "mod@example.com".to_string(),
            username: Some("mod_user".to_string()),
            permissions: vec!["delete".to_string(), "edit".to_string()],
            ..Default::default()
        },
    };
    let _response = create_moderator(configuration, params).await?;
    Ok(())
}
[inline-code-end]