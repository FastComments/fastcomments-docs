## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantBody | CreateTenantBody | 是 |  |

## 响应

返回：[`CreateTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenant200Response.ts)

## 示例

[inline-code-attrs-start title = 'createTenant 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corporation";
const billing: BillingInfo = { planId: "pro", billingContactEmail: "finance@acme-corp.com", currency: "USD" };
const domainConfig: APIDomainConfiguration = { primaryDomain: "comments.acme-corp.com", allowedDomains: ["acme-corp.com", "www.acme-corp.com"], enforceHttps: true };
const importedSites: ImportedSiteType[] = [{ siteId: "site-001", url: "https://blog.acme-corp.com", name: "Acme Blog" }]; // optional
const createBody: CreateTenantBody = { tenantName: "Acme Corporation", adminEmail: "admin@acme-corp.com", billingInfo: billing, domainConfiguration: domainConfig, importedSites, enableModeration: true };
const response: CreateTenant200Response = await createTenant(tenantId, createBody);
[inline-code-end]

---