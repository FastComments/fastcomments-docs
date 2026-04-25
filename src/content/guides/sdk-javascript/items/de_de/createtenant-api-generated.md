## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createTenantBody | CreateTenantBody | Ja |  |

## Antwort

Gibt zurück: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenant200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'createTenant Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-001';
const createTenantBody: CreateTenantBody = {
  name: 'Acme Corporation',
  domainConfiguration: { primaryDomain: 'comments.acme.com', enforceHttps: true } as APIDomainConfiguration,
  billingInfo: { planId: 'enterprise', contactEmail: 'billing@acme.com' } as BillingInfo
  // optionale Felder wie ssoConfig oder customConfig wurden absichtlich weggelassen
} as CreateTenantBody;

const result: CreateTenant200Response = await createTenant(tenantId, createTenantBody);
[inline-code-end]

---