## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## תגובה

מחזיר: [`DeleteTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteTenantPackageResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת deleteTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function removeTenantPackage(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const packageId: string = "pkg_67890";

  const result: DeleteTenantPackageResponse = await deleteTenantPackage(tenantId, packageId);
  // השתמש בתוצאה לפי הצורך
}

removeTenantPackage();
[inline-code-end]