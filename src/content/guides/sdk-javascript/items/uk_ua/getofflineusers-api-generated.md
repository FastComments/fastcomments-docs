Коментатори, які раніше залишали коментарі на сторінці, але наразі НЕ в мережі. Відсортовані за displayName.
Використовуйте це після вичерпання /users/online для відображення розділу «Учасники».
Курсорна пагінація за commenterName: сервер проходить частковий індекс {tenantId, urlId, commenterName}
від afterName вперед через $gt, без витрат на $skip.

## Параметри

| Назва | Тип | Обов'язковий | Опис |
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