## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| badgeId | string | Да |  |
| userId | string | Не |  |
| commentId | string | Не |  |
| broadcastId | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AwardUserBadgeResponse.ts)

## Пример

[inline-code-attrs-start title = 'putAwardBadge Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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