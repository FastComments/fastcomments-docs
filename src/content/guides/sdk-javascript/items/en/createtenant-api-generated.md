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
const tenantId: string = 'tenant-2026-nyc';

const createTenantBody: CreateTenantBody = {
  displayName: 'Acme Corporation',
  primaryDomain: 'acme.com',
  billingInfo: { plan: 'enterprise', billingContactEmail: 'billing@acme.com' },
  importedSites: [{ siteId: 'site-001', url: 'https://blog.acme.com' }] // optional field included; other optional fields omitted
} as CreateTenantBody;

const result: CreateTenant200Response = await createTenant(tenantId, createTenantBody);
[inline-code-end]
