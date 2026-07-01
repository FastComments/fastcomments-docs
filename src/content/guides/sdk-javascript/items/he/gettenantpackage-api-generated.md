## פרמטרים

| שם | טיפוס | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |

## תגובה

מחזירה: [`GetTenantPackageResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackageResponse1.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTenantPackage(): Promise<void> {
    const tenantId: string = "acme-tenant-2024";
    const packageId: string = "premium-package-01";
    const response: GetTenantPackageResponse1 = await getTenantPackage(tenantId, packageId);

    // שדות אופציונליים בתגובה
    const tenantPackage: TenantPackage | undefined = response.tenantPackage;
    const customConfig: CustomConfigParameters | undefined = response.customConfigParameters;
}
[inline-code-end]