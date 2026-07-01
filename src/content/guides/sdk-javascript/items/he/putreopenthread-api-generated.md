## פרמטרים

| שם | סוג | הנדרש | תיאור |
|------|------|----------|-------------|
| urlId | string | כן |  |
| tenantId | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`PutReopenThreadResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutReopenThreadResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל‑putReopenThread'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleUsage() {
  const urlId: string = "thread-9f8b7c6a";
  const tenantId: string = "tenant-001";
  const sso: string = "sso-3f9d2e1a";

  const resultAll: PutReopenThreadResponse = await putReopenThread(urlId, tenantId, sso);
  console.log(resultAll);

  const resultMinimal: PutReopenThreadResponse = await putReopenThread(urlId);
  console.log(resultMinimal);
}
[inline-code-end]