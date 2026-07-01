## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`GetCountsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCountsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getCounts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
    const tenantId: string = "acme-corp";
    const ssoToken: string = "sso-token-2024";

    const withBoth: GetCountsResponse = await getCounts(tenantId, ssoToken);
    const withTenantOnly: GetCountsResponse = await getCounts(tenantId);
    const withoutParams: GetCountsResponse = await getCounts();

    console.log(withBoth, withTenantOnly, withoutParams);
}
runExample();
[inline-code-end]