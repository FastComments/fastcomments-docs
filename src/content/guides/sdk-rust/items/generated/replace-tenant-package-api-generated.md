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
async fn run_replace_package() -> Result<FlagCommentPublic200Response, Error> {
    let params: ReplaceTenantPackageParams = ReplaceTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "package-basic-2026".to_string(),
        replace_tenant_package_body: models::ReplaceTenantPackageBody {
            name: "Moderation Basic".to_string(),
            description: Some("Standard moderation package for news sites".to_string()),
            enabled: Some(true),
            plan: Some("standard".to_string()),
            custom_config_parameters: Some(models::CustomConfigParameters {
                max_comment_length: Some(1000),
                allow_images: Some(true),
            }),
            vote_style: Some(models::VoteStyle::Thumbs),
        },
    };
    let response = replace_tenant_package(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
