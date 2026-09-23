Past commenters on the page who are NOT currently online. Sorted by displayName.  
Попередні коментатори на сторінці, які НЕ перебувають онлайн. Відсортовано за displayName.

Use this after exhausting /users/online to render a "Members" section.  
Використовуйте це після завершення запитів /users/online, щоб відобразити розділ "Members".

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
Курсорна пагінація за commenterName: сервер проходить частковий індекс {tenantId, urlId, commenterName} від afterName вперед за допомогою $gt, без витрат на $skip.

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| afterName | string | Ні |  |
| afterUserId | string | Ні |  |

## Відповідь

Повертає: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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