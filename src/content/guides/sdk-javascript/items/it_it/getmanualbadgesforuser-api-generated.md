## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| badgesUserId | string | No |  |
| commentId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`GetManualBadgesForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetManualBadgesForUserResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getManualBadgesForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const userId: string = "user_42";
  const commentId: string = "comment_1001";
  const tenantId: string = "tenant_acme";
  const ssoToken: string = "sso_5f6g7h8i9j";

  const badges: GetManualBadgesForUserResponse = await getManualBadgesForUser(userId, commentId, tenantId, ssoToken);
  const limitedBadges: GetManualBadgesForUserResponse = await getManualBadgesForUser(userId);
})();
[inline-code-end]