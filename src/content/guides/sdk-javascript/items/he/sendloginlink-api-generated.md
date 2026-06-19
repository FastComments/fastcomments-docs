## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| redirectURL | string | לא |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-sendLoginLink'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async function run(): Promise<void> {
  const tenantId: string = 'fc_tenant_9f3b2c';
  const id: string = 'user_42b7f';
  const redirectURL: string = 'https://dashboard.acme-corp.com/welcome';
  const responseWithoutRedirect: APIEmptyResponse = await sendLoginLink(tenantId, id);
  const responseWithRedirect: APIEmptyResponse = await sendLoginLink(tenantId, id, redirectURL);
  console.log(responseWithoutRedirect, responseWithRedirect);
})();
[inline-code-end]

---