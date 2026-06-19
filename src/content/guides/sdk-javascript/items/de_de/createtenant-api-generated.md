## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createTenantBody | CreateTenantBody | Ja |  |

## Antwort

Gibt zurück: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'createTenant Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-001';
const createTenantBody: CreateTenantBody = {
  name: 'Acme Corporation',
  adminEmail: 'platform-admin@acme.com',
  apiDomainConfiguration: { domain: 'comments.acme.com', tlsEnabled: true },
  importedSites: [{ siteId: 'main-site', url: 'https://www.acme.com', platform: 'nextjs' }],
  billingInfo: { plan: 'enterprise', billingEmail: 'billing@acme.com', vatId: 'GB123456789' }
};
const result: CreateTenantResponse = await createTenant(tenantId, createTenantBody);
[inline-code-end]

---