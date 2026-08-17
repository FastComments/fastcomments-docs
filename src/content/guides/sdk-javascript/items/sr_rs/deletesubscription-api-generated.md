## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |

## Одговор

Враћа: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteSubscriptionAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'deleteSubscription Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_9876";
  const subscriptionId: string = "sub_54321";
  const userId: string = "user_abc123";

  // Са опционом userId
  const responseWithUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, subscriptionId, userId);

  // Без опционог userId
  const responseWithoutUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, subscriptionId);
})();
[inline-code-end]