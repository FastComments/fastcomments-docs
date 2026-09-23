## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Отговор

Връща: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCachedNotificationCountResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getCachedNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const id: string = "user_9876";
  const response: GetCachedNotificationCountResponse = await getCachedNotificationCount(tenantId, id);
})();
[inline-code-end]

---