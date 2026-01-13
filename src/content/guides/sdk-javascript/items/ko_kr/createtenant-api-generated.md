## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createTenantBody | CreateTenantBody | 예 |  |

## 응답

반환: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenant200Response.ts)

## 예제

[inline-code-attrs-start title = 'createTenant 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corporation";
const billing: BillingInfo = { planId: "pro", billingContactEmail: "finance@acme-corp.com", currency: "USD" };
const domainConfig: APIDomainConfiguration = { primaryDomain: "comments.acme-corp.com", allowedDomains: ["acme-corp.com", "www.acme-corp.com"], enforceHttps: true };
const importedSites: ImportedSiteType[] = [{ siteId: "site-001", url: "https://blog.acme-corp.com", name: "Acme Blog" }]; // 선택적
const createBody: CreateTenantBody = { tenantName: "Acme Corporation", adminEmail: "admin@acme-corp.com", billingInfo: billing, domainConfiguration: domainConfig, importedSites, enableModeration: true };
const response: CreateTenant200Response = await createTenant(tenantId, createBody);
[inline-code-end]