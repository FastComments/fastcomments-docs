## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createTenantBody | CreateTenantBody | 是 |  |

## 响应

返回: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantResponse.ts)

## 示例

[inline-code-attrs-start title = 'createTenant 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "acme-corp-2024";
  const createTenantBody: CreateTenantBody = {
    name: "Acme Corp",
    domainConfiguration: { domain: "comments.acme.com", sslEnabled: true },
    importedSite: { siteId: "site-123", source: "wordpress" },
    billingInfo: { plan: "enterprise", renewalDate: "2025-01-01" },
    description: "Tenant for Acme Corp's production environment", // 可选
  };
  const response: CreateTenantResponse = await createTenant(tenantId, createTenantBody);
}
run();
[inline-code-end]