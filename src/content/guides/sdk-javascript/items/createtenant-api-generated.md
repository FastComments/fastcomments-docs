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
const domainConfiguration: APIDomainConfiguration = { primaryDomain: "comments.acme.com", redirectDomains: ["www.acme.com"], enforceHttps: true };
const billingInfo: BillingInfo = { contactEmail: "billing@acme.com", planId: "enterprise-monthly", billingAddress: "1 Acme Way, Metropolis" };
const importedSite: ImportedSiteType = { siteId: "site-42", url: "https://www.acme.com", title: "Acme Main Site" };
const createTenantBody: CreateTenantBody = {
  name: "Acme Corporation",
  domainConfiguration,
  billingInfo,
  importedSites: [importedSite], // optional: include imported site during creation
  enableSSO: true
};
const result: CreateTenant200Response = await createTenant(tenantId, createTenantBody);
[inline-code-end]
