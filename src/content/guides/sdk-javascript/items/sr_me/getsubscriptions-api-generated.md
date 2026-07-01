## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |

## Одговор

Враћа: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSubscriptionsAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getSubscriptions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-123";
    const userId: string = "user-789";

    const responseWithUser: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId, userId);
    const responseWithoutUser: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId);
})();
[inline-code-end]