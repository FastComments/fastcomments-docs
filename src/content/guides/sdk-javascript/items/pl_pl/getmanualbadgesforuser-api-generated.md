## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| badgesUserId | string | Nie |  |
| commentId | string | Nie |  |
| tenantId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`GetManualBadgesForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetManualBadgesForUserResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getManualBadgesForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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