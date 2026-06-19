## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const idPrefix: string | undefined = 'count';
const notificationId: string = `${idPrefix ? idPrefix + '-' : ''}8b3a9f6c-3e8f-4f6a-a2f3-1a2b3c4d5e6f`;
const tenantId: string = 'acme-media-tenant-42';
const result: APIEmptyResponse = await deleteNotificationCount(tenantId, notificationId);
[inline-code-end]

---