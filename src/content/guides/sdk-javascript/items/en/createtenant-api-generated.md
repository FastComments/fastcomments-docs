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
const tenantId: string = "acme-enterprises-001";
const domainConfig: APIDomainConfiguration = { primaryDomain: "acme-enterprises.com", redirectToPrimary: true };
const billingInfo: BillingInfo = { planId: "business-monthly", billingContactEmail: "finance@acme-enterprises.com" };
const initialSite: ImportedSiteType = { siteName: "Main Site", siteUrl: "https://acme-enterprises.com" };
const tosConfig: TOSConfig = { required: true, url: "https://acme-enterprises.com/terms" };

const createTenantBody: CreateTenantBody = {
  name: "Acme Enterprises",
  description: "Primary tenant for Acme Enterprises",
  domainConfiguration: domainConfig,
  billingInfo,
  initialSite, // optional parameter demonstrated
  tosConfig
};

const result: CreateTenant200Response = await createTenant(tenantId, createTenantBody);
[inline-code-end]
