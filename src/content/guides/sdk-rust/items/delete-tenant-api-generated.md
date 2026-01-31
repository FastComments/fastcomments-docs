## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| sure | String | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'delete_tenant Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<FlagCommentPublic200Response, Error> {
    let params: DeleteTenantParams = DeleteTenantParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "tenant-12345".to_string(),
        sure: Some("true".to_string()),
    };
    let resp: FlagCommentPublic200Response = delete_tenant(configuration, params).await?;
    Ok(resp)
}
[inline-code-end]
