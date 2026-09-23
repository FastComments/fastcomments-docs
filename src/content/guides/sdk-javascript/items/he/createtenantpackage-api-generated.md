## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantPackageBody | CreateTenantPackageBody | Yes |  |

## תגובה

מחזיר: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackageResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת createTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_9f8b7c6d";
  const createTenantPackageBody: CreateTenantPackageBody = {
    name: "Enterprise Package",
    priceCents: 49999,
    // שדה אופציונלי
    description: "Full suite of enterprise features"
  };
  const result: CreateTenantPackageResponse = await createTenantPackage(tenantId, createTenantPackageBody);
  console.log(result);
}
runExample();
[inline-code-end]

---