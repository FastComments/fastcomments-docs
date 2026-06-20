## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| badgeId | string | Sim |  |
| userId | string | Não |  |
| commentId | string | Não |  |
| broadcastId | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RemoveUserBadgeResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de putRemoveBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const badgeId: string = 'badge_7392';
  const userId: string = 'user_1284';
  const commentId: string = 'cmt_5583';
  const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyXzEyODQifQ.signature';
  const response: RemoveUserBadgeResponse = await putRemoveBadge(badgeId, userId, commentId, undefined, sso);
  console.log(response);
})();
[inline-code-end]

---