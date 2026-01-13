## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Response

Връща: [`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCachedNotificationCount200Response.ts)

## Пример

[inline-code-attrs-start title = 'getCachedNotificationCount Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-01';
const baseNotificationId: string = 'notif-000123';
const idSuffix: string | undefined = undefined; // пример за незадължителен параметър
const notificationId: string = idSuffix ? `${baseNotificationId}-${idSuffix}` : baseNotificationId;
const result: GetCachedNotificationCount200Response = await getCachedNotificationCount(tenantId, notificationId);
console.log(result);
[inline-code-end]

---