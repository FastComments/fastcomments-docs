## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentIds | string | Yes |  |
| sso | string | No |  |

## Odgovor

Vraća: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CheckedCommentsForBlocked200Response.ts)

## Primjer

[inline-code-attrs-start title = 'checkedCommentsForBlocked Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_fa3b2c9e';
  const commentIds: string = 'cmt_112233,cmt_445566';
  const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI0Njc4IiwidGVuYW50IjoidGVuYW50X2ZhM2IifQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
  const resultWithSSO: CheckedCommentsForBlocked200Response = await checkedCommentsForBlocked(tenantId, commentIds, sso);
  const resultWithoutSSO: CheckedCommentsForBlocked200Response = await checkedCommentsForBlocked(tenantId, commentIds);
  console.log(resultWithSSO, resultWithoutSSO);
})();
[inline-code-end]

---