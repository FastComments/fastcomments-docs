Минали коментатори на страницата, които в момента НЕ са онлайн. Подредени по displayName.
Използвайте това след изчерпване на /users/online, за да визуализирате секция "Членове".
Пагинация с курсор по commenterName: сървърът обхожда частичния индекс {tenantId, urlId, commenterName} от afterName напред чрез $gt, без разход за $skip.

## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| afterName | string | Не |  |
| afterUserId | string | Не |  |

## Отговор

Връща: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const urlId: string = "page_9876";
  const afterName: string = "John Doe";
  const afterUserId: string = "user_abc123";

  const offlineResponse: PageUsersOfflineResponse = await getOfflineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(offlineResponse);
}
[inline-code-end]