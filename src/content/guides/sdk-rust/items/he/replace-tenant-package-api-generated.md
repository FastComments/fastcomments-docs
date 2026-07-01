## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| replace_tenant_package_body | models::ReplaceTenantPackageBody | Yes |  |

## תגובה

מחזיר: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## דוגמה

[inline-code-attrs-start title = 'replace_tenant_package דוגמה'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = ReplaceTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article".to_string(),
        replace_tenant_package_body: models::ReplaceTenantPackageBody {
            package_id: "premium-plan".to_string(),
            enabled: true,
            description: Some("Premium package for high traffic".to_string()),
        },
    };
    replace_tenant_package(&configuration, params).await?;
    Ok(())
}
[inline-code-end]