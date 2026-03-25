---
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
const tenantId: string = "acme-corp-001";
const createTenantBody: CreateTenantBody = {
  name: "Acme Corporation",
  domain: "comments.acme.com",
  adminContact: { name: "Jane Doe", email: "jane.doe@acme.com" },
  billingInfo: { planId: "pro-monthly", billingContactEmail: "billing@acme.com" },
  importedSite: { siteId: "site-123", siteName: "Acme Blog" } // 선택적으로 가져온 사이트
};
const result: CreateTenant200Response = await createTenant(tenantId, createTenantBody);
[inline-code-end]

---