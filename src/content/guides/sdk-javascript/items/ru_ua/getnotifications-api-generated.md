## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Нет |  |
| urlId | string | Нет |  |
| fromCommentId | string | Нет |  |
| viewed | boolean | Нет |  |
| type | string | Нет |  |
| skip | number | Нет |  |

## Ответ

Возвращает: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_98b3f';
const userId: string = 'user_8a3f';
const urlId: string = '/blog/2026/new-feature';
const viewed: boolean = false;
const type: string = 'reply';
const skip: number = 10;
const notifications: GetNotificationsResponse = await getNotifications(tenantId, userId, urlId, undefined, viewed, type, skip);
[inline-code-end]

---