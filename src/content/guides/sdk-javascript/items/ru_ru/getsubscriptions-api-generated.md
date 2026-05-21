## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Нет |  |

## Ответ

Возвращает: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSubscriptionsAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getSubscriptions'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "contoso-9a1b2c";
const userId: string = "u-482f6";
const subscriptions: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId);
const userSubscriptions: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId, userId);
[inline-code-end]

---