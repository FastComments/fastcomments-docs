## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| domain | string | כן |  |

## תגובה

מחזיר: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteDomainConfigResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של deleteDomainConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "org_5b2f9c3a";
const domain: string = "comments.acme-corp.com";
const result: DeleteDomainConfigResponse = await deleteDomainConfig(tenantId, domain);
[inline-code-end]

---