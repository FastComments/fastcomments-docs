## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| value | string | לא |  |
| tenantId | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`GetSearchPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSearchPagesResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getSearchPages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const query: string = "network outage";
  const tenantId: string = "tenant-9876";
  const ssoToken: string = "sso-abc123def456";

  const searchResult: GetSearchPagesResponse = await getSearchPages(query, tenantId, ssoToken);
  const searchResultNoSso: GetSearchPagesResponse = await getSearchPages(query, tenantId);
})();
[inline-code-end]

---