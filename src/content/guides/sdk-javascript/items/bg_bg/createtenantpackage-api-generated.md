## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantPackageBody | CreateTenantPackageBody | Yes |  |

## Отговор

Връща: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример на createTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme-corp_001";
const createTenantPackageBody: CreateTenantPackageBody = {
  name: "Acme Standard Package",
  description: "Default package for Acme Corp comments with moderation and SSO enabled",
  enabled: true,
  maxCommentsPerThread: 500,
  voteStyle: "thumbs",
  gifRating: "PG-13",
  tosConfig: { enabled: true, url: "https://acme.example.com/terms" } // optional parameter demonstrated
};
const result: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
[inline-code-end]

---