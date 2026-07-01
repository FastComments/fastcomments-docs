## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Yes |  |

## תגובה

מחזיר: [`ReplaceTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReplaceTenantPackageResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'replaceTenantPackage דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-tenant-01";
    const packageId: string = "pkg-2024-annual";

    const config: CustomConfigParameters = {
        // שדות קונפיגורציה מותאמים כאן
    };

    const body: ReplaceTenantPackageBody = {
        name: "Enterprise Package",
        // קונפיגורציה מותאמת אופציונלית
        customConfig: config,
    };

    const response: ReplaceTenantPackageResponse = await replaceTenantPackage(tenantId, packageId, body);
    console.log(response);
})();
[inline-code-end]