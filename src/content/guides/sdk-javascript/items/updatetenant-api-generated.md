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
const tenantId: string = "tenant_84a7c2f9";
const id: string = "site_7b2d1a9e";
const billingInfo: BillingInfo = { plan: "business", paymentMethod: "card_visa", nextBillingDate: "2026-03-01" };
const domainConfig: APIDomainConfiguration = { hostname: "comments.example.org", secure: true };
const updateTenantBody: UpdateTenantBody = {
  name: "Example Comments Tenant",
  billing: billingInfo,           // optional: update billing info
  domains: [domainConfig],        // optional: add domain configuration
  active: true
};
const result: FlagCommentPublic200Response = await updateTenant(tenantId, id, updateTenantBody);
[inline-code-end]
