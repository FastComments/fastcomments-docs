## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Odgovor

Vrne: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptySuccessResponse.ts)

## Primer

[inline-code-attrs-start title = 'deleteUserBadge Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const badgeId: string = "moderator-badge";
  const result: APIEmptySuccessResponse = await deleteUserBadge(tenantId, badgeId);
  console.log(result);
})();
[inline-code-end]