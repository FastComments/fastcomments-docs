## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| createTenantBody | CreateTenantBody | Evet |  |

## Yanıt

Döndürür: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenant200Response.ts)

## Örnek

[inline-code-attrs-start title = 'createTenant Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-001';
const createTenantBody: CreateTenantBody = {
  name: 'Acme Corporation',
  domainConfiguration: { primaryDomain: 'comments.acme.com', enforceHttps: true } as APIDomainConfiguration,
  billingInfo: { planId: 'enterprise', contactEmail: 'billing@acme.com' } as BillingInfo
  // ssoConfig veya customConfig gibi isteğe bağlı alanlar kasıtlı olarak atlanmıştır
} as CreateTenantBody;

const result: CreateTenant200Response = await createTenant(tenantId, createTenantBody);
[inline-code-end]