## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_tenant_package_body | models::UpdateTenantPackageBody | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'update_tenant_package Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update() -> Result<(), Error> {
    let params: UpdateTenantPackageParams = UpdateTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "package-premium".to_string(),
        update_tenant_package_body: models::UpdateTenantPackageBody {
            name: Some("Premium Moderation".to_string()),
            description: Some("Priority moderation, AI filtering, GIF support".to_string()),
            billing_cycle: Some("monthly".to_string()),
            features: Some(vec![
                "priority-moderation".to_string(),
                "ai-filtering".to_string(),
                "gif-support".to_string(),
            ]),
            enabled: Some(true),
        },
    };

    let response: FlagCommentPublic200Response =
        update_tenant_package(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]
