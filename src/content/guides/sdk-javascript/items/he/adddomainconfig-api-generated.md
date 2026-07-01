## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| addDomainConfigParams | AddDomainConfigParams | כן |  |

## תגובה

מחזיר: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddDomainConfigResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'addDomainConfig דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_12345';
  const config: AddDomainConfigParams = {
    domain: 'myblog.example.com',
    enabled: true,
    // התיאור הוא אופציונלי ונמנע כאן
  };
  const response: AddDomainConfigResponse = await addDomainConfig(tenantId, config);
  console.log(response);
})();
[inline-code-end]