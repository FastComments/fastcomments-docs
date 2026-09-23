## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| badgeId | string | Sim |  |
| userId | string | Não |  |
| commentId | string | Não |  |
| broadcastId | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AwardUserBadgeResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo putAwardBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function awardBadgeExample(): Promise<void> {
  const tenantId: string = "tenant_42";
  const badgeId: string = "badge_superstar";
  const userId: string = "user_1001";
  const commentId: string = "comment_2023";

  const result: AwardUserBadgeResponse = await putAwardBadge(
    tenantId,
    badgeId,
    userId,
    commentId
  );

  console.log(result);
}
[inline-code-end]

---