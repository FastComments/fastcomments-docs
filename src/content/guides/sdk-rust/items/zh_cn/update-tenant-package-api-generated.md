## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| update_tenant_package_body | models::UpdateTenantPackageBody | 是 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 示例

[inline-code-attrs-start title = 'update_tenant_package 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_update_tenant_package() -> Result<FlagCommentPublic200Response, Error> {
    let params: UpdateTenantPackageParams = UpdateTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "pro-plan-2026".to_string(),
        update_tenant_package_body: models::UpdateTenantPackageBody {
            name: Some("Pro Plan".to_string()),
            description: Some("Priority support, custom branding, and advanced moderation tools".to_string()),
            enabled: Some(true),
            monthly_price_cents: Some(1999),
            features: Some(vec![
                "priority_support".to_string(),
                "custom_branding".to_string(),
                "advanced_moderation".to_string(),
            ]),
        },
    };

    let response: FlagCommentPublic200Response = update_tenant_package(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---