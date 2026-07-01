## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |

## Відповідь

Повертає: [`DeleteUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteUserBadgeResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleDeleteBadge(): Promise<void> {
  const tenantId: string = "tenant_9f8b7c6a";
  const badgeId: string = "badge_4e3d2c1b";
  const result: DeleteUserBadgeResponse = await deleteUserBadge(tenantId, badgeId);
  console.log(result);
}
exampleDeleteBadge();
[inline-code-end]