## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | Yes |  |
| createTenantBody | CreateTenantBody | Yes |  |

## 응답

반환: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantResponse.ts)

## 예시

[inline-code-attrs-start title = 'createTenant 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "acme-corp-2024";
  const createTenantBody: CreateTenantBody = {
    name: "Acme Corp",
    domainConfiguration: { domain: "comments.acme.com", sslEnabled: true },
    importedSite: { siteId: "site-123", source: "wordpress" },
    billingInfo: { plan: "enterprise", renewalDate: "2025-01-01" },
    description: "Tenant for Acme Corp's production environment", // 선택 사항
  };
  const response: CreateTenantResponse = await createTenant(tenantId, createTenantBody);
}
run();
[inline-code-end]