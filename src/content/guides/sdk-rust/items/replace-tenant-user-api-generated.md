## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| replace_tenant_user_body | models::ReplaceTenantUserBody | Yes |  |
| update_comments | String | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'replace_tenant_user Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let replace_body: models::ReplaceTenantUserBody = models::ReplaceTenantUserBody {
    username: "jane.doe".to_string(),
    email: "jane.doe@acme-news.com".to_string(),
    display_name: Some("Jane Doe".to_string()),
    roles: Some(vec!["editor".to_string(), "contributor".to_string()]),
};
let params: ReplaceTenantUserParams = ReplaceTenantUserParams {
    tenant_id: "acme-news-tenant".to_string(),
    id: "user-7890".to_string(),
    replace_tenant_user_body: replace_body,
    update_comments: Some("true".to_string()),
};
let response: FlagCommentPublic200Response = replace_tenant_user(&configuration, params).await?;
[inline-code-end]
