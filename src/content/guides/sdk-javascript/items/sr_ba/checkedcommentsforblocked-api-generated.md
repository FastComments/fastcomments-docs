## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| commentIds | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`CheckedCommentsForBlockedResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CheckedCommentsForBlockedResponse.ts)

## Primjer

[inline-code-attrs-start title = 'checkedCommentsForBlocked Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-123";
  const commentIds: string = "cmt_001,cmt_002";
  const ssoToken: string = "ssoTokenXYZ";

  const blockedCheck: CheckedCommentsForBlockedResponse = await checkedCommentsForBlocked(tenantId, commentIds);
  const blockedCheckWithSso: CheckedCommentsForBlockedResponse = await checkedCommentsForBlocked(tenantId, commentIds, ssoToken);
})();
[inline-code-end]