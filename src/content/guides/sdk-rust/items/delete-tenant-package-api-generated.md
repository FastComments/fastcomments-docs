## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'delete_tenant_package Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_package() -> Result<FlagCommentPublic200Response, Error> {
    let params: DeleteTenantPackageParams = DeleteTenantPackageParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("news/article-package-2026-01"),
    };
    let response: FlagCommentPublic200Response = delete_tenant_package(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
