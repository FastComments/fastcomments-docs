## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| createTenantBody | CreateTenantBody | Sì |  |

## Risposta

Restituisce: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenant200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di createTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corporation";
const billing: BillingInfo = { planId: "pro", billingContactEmail: "finance@acme-corp.com", currency: "USD" };
const domainConfig: APIDomainConfiguration = { primaryDomain: "comments.acme-corp.com", allowedDomains: ["acme-corp.com", "www.acme-corp.com"], enforceHttps: true };
const importedSites: ImportedSiteType[] = [{ siteId: "site-001", url: "https://blog.acme-corp.com", name: "Acme Blog" }]; // opzionale
const createBody: CreateTenantBody = { tenantName: "Acme Corporation", adminEmail: "admin@acme-corp.com", billingInfo: billing, domainConfiguration: domainConfig, importedSites, enableModeration: true };
const response: CreateTenant200Response = await createTenant(tenantId, createBody);
[inline-code-end]

---