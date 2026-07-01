## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| badgesUserId | string | Não |  |
| commentId | string | Não |  |
| tenantId | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`GetManualBadgesForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetManualBadgesForUserResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getManualBadgesForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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