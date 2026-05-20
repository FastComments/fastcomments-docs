## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantBody | UpdateTenantBody | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'updateTenant Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8f1b2";
const id: string = "site_4c9d";
const updateTenantBody: UpdateTenantBody = {
  billingInfo: { planId: "enterprise", contactEmail: "billing@acme.com" } as BillingInfo,
  apiDomainConfiguration: { domain: "acme-app.prod", allowedOrigins: ["https://app.acme.com"] } as APIDomainConfiguration
} as UpdateTenantBody;
const result: FlagCommentPublic200Response = await updateTenant(tenantId, id, updateTenantBody);
[inline-code-end]
