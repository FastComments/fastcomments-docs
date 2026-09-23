## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| meta | string | לא |  |
| skip | number | לא |  |

## תגובה

מחזיר: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getTenants'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
    const tenantId: string = "c3f5e9b2-4d1a-4f2b-9a6e-1234567890ab";
    const meta: string = "includeBilling";
    const skip: number = 10;

    const result: GetTenantsResponse = await getTenants(tenantId, meta, skip);
    console.log(result);
}
runExample();
[inline-code-end]