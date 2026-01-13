---
## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| create_tenant_package_body | models::CreateTenantPackageBody | 是 |  |

## 回應

回傳：[`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_package_200_response.rs)

## 範例

[inline-code-attrs-start title = 'create_tenant_package 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_create_package() -> Result<(), Error> {
    let params: CreateTenantPackageParams = CreateTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_tenant_package_body: models::CreateTenantPackageBody {
            name: "Acme News Package".to_string(),
            description: Some("Moderated comments for Acme News articles".to_string()),
            plan: Some("standard".to_string()),
            allow_gifs: Some(true),
            gif_rating: Some(GifRating::GeneralAudience),
            image_content_profanity_level: Some(ImageContentProfanityLevel::Moderate),
            sso_security_level: Some(SsoSecurityLevel::Strict),
            custom_config: Some(CustomConfigParameters {
                max_comment_length: Some(1000),
                require_moderation: Some(true),
            }),
        },
    };
    let response: CreateTenantPackage200Response = create_tenant_package(&configuration, params).await?;
    let _package: TenantPackage = response.0;
    Ok(())
}
[inline-code-end]

---