---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|--------------|--------------|
| tenantId | string | Ja |  |
| createTenantBody | CreateTenantBody | Ja |  |

## Antwort

Gibt zurück: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenant200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'createTenant Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-001";
const createTenantBody: CreateTenantBody = {
  name: "Acme Corporation",
  domain: "comments.acme.com",
  adminContact: { name: "Jane Doe", email: "jane.doe@acme.com" },
  billingInfo: { planId: "pro-monthly", billingContactEmail: "billing@acme.com" },
  importedSite: { siteId: "site-123", siteName: "Acme Blog" } // optionale importierte Website
};
const result: CreateTenant200Response = await createTenant(tenantId, createTenantBody);
[inline-code-end]

---