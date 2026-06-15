---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| createTenantBody | CreateTenantBody | Evet |  |

## Yanıt

Döndürür: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenant200Response.ts)

## Örnek

[inline-code-attrs-start title = 'createTenant Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-news-01';
const createTenantBody: CreateTenantBody = {
  name: 'Acme News',
  domainConfiguration: { primaryDomain: 'news.acme.com', redirectHttps: true } as APIDomainConfiguration,
  importedSites: [{ siteId: 'site-92', url: 'https://news.acme.com' }] as ImportedSiteType[],
  billingInfo: { planId: 'business_monthly', contactEmail: 'billing@acme.com' } as BillingInfo
};
const result: CreateTenant200Response = await createTenant(tenantId, createTenantBody);
[inline-code-end]

---