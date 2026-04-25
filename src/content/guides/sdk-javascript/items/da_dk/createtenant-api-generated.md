## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createTenantBody | CreateTenantBody | Ja |  |

## Svar

Returnerer: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenant200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'createTenant Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-001';
const createTenantBody: CreateTenantBody = {
  name: 'Acme Corporation',
  domainConfiguration: { primaryDomain: 'comments.acme.com', enforceHttps: true } as APIDomainConfiguration,
  billingInfo: { planId: 'enterprise', contactEmail: 'billing@acme.com' } as BillingInfo
  // valgfri felter som ssoConfig eller customConfig er med vilje udeladt
} as CreateTenantBody;

const result: CreateTenant200Response = await createTenant(tenantId, createTenantBody);
[inline-code-end]

---