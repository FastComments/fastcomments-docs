## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| createTenantPackageBody | CreateTenantPackageBody | כן |  |

## תגובה

מחזיר: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-createTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme-corp_001";
const createTenantPackageBody: CreateTenantPackageBody = {
  name: "Acme Standard Package",
  description: "Default package for Acme Corp comments with moderation and SSO enabled",
  enabled: true,
  maxCommentsPerThread: 500,
  voteStyle: "thumbs",
  gifRating: "PG-13",
  tosConfig: { enabled: true, url: "https://acme.example.com/terms" } // פרמטר אופציונלי (להדגמה)
};
const result: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
[inline-code-end]

---