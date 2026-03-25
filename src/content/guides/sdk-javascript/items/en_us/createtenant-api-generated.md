## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantBody | CreateTenantBody | Yes |  |

## Response

Returns: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenant200Response.ts)

## Example

[inline-code-attrs-start title = 'createTenant Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-001";
const createTenantBody: CreateTenantBody = {
  name: "Acme Corporation",
  domain: "comments.acme.com",
  adminContact: { name: "Jane Doe", email: "jane.doe@acme.com" },
  billingInfo: { planId: "pro-monthly", billingContactEmail: "billing@acme.com" },
  importedSite: { siteId: "site-123", siteName: "Acme Blog" } // optional imported site
};
const result: CreateTenant200Response = await createTenant(tenantId, createTenantBody);
[inline-code-end]
