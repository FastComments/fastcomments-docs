---
## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createTenantBody | CreateTenantBody | Да |  |

## Ответ

Возвращает: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenant200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример createTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-001';
const createTenantBody: CreateTenantBody = {
  name: 'Acme Corporation',
  domainConfiguration: { primaryDomain: 'comments.acme.com', enforceHttps: true } as APIDomainConfiguration,
  billingInfo: { planId: 'enterprise', contactEmail: 'billing@acme.com' } as BillingInfo
  // необязательные поля, такие как ssoConfig или customConfig, намеренно опущены
} as CreateTenantBody;

const result: CreateTenant200Response = await createTenant(tenantId, createTenantBody);
[inline-code-end]

---