## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_tenant_body | models::UpdateTenantBody | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Example

[inline-code-attrs-start title = 'update_tenant Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: UpdateTenantParams = UpdateTenantParams {
    tenant_id: "acme-corp-tenant".to_string(),
    id: "site-news-124".to_string(),
    update_tenant_body: models::UpdateTenantBody {
        name: Some("Acme Corp".to_string()),
        domain: Some("comments.acme.example.com".to_string()),
        billing_info: Some(models::BillingInfo {
            plan: Some("pro".to_string()),
            contact_email: Some("billing@acme.example.com".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    },
};

let response: FlagCommentPublic200Response = update_tenant(&configuration, params).await?;
[inline-code-end]
