## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| replace_tenant_package_body | models::ReplaceTenantPackageBody | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'replace_tenant_package Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = ReplaceTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news-standard-package".to_string(),
        replace_tenant_package_body: models::ReplaceTenantPackageBody {
            name: "News Standard".to_string(),
            description: Some("Standard moderation package for news sites".to_string()),
            active: true,
            max_comments_per_minute: Some(120),
            gif_rating: Some(GifRating::Moderate),
            sso_security_level: Some(SsoSecurityLevel::Strict),
            custom_config_parameters: Some(CustomConfigParameters {
                moderation_threshold: Some(0.7),
                enable_auto_ban: Some(false),
            }),
        },
    };

    let response: FlagCommentPublic200Response = replace_tenant_package(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
