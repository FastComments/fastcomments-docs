---
## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|--------------|------|
| badgeId | string | Так |  |
| userId | string | Ні |  |
| commentId | string | Ні |  |
| broadcastId | string | Ні |  |
| tenantId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`PutRemoveBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutRemoveBadgeResponse.ts)

## Приклад

[inline-code-attrs-start title = 'putRemoveBadge Приклад'; type = 'typescript'; isFunctional false; inline-code-attrs-end]
[inline-code-start]
const badgeId: string = "badge-12345";
const userId: string = "user-9876";
const commentId: string = "comment-5555";
const broadcastId: string = "broadcast-001";

const result: PutRemoveBadgeResponse = await putRemoveBadge(
  badgeId,
  userId,
  commentId,
  broadcastId
);
[inline-code-end]

---