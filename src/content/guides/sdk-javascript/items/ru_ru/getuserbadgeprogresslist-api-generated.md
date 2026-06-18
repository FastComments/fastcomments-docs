## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Нет |  |
| limit | number | Нет |  |
| skip | number | Нет |  |

## Ответ

Возвращает: [`GetUserBadgeProgressList200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressList200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример использования getUserBadgeProgressList'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f3a2b9c';
const userId: string = 'user_7721d';
const limit: number = 20;
const skip: number = 0;
const result: GetUserBadgeProgressList200Response = await getUserBadgeProgressList(tenantId, userId, limit, skip);
[inline-code-end]

---