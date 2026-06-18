## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Не |  |
| limit | number | Не |  |
| skip | number | Не |  |

## Одговор

Враћа: [`GetUserBadgeProgressList200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressList200Response.ts)

## Пример

[inline-code-attrs-start title = 'getUserBadgeProgressList пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f3a2b9c';
const userId: string = 'user_7721d';
const limit: number = 20;
const skip: number = 0;
const result: GetUserBadgeProgressList200Response = await getUserBadgeProgressList(tenantId, userId, limit, skip);
[inline-code-end]

---