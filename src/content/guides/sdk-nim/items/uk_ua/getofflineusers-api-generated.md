---
Попередні коментатори на сторінці, які наразі не онлайн. Відсортовано за displayName.
Використовуйте це після вичерпання /users/online, щоб відобразити розділ "Учасники".
Курсорна пагінація за commenterName: сервер проходить частковий індекс {tenantId, urlId, commenterName}
від afterName вперед за допомогою $gt, без витрат на $skip.

## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| afterName | string | Ні |  |
| afterUserId | string | Ні |  |

## Відповідь

Повертає: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-how-to-code",
  afterName = "",
  afterUserId = ""
)

if response.isSome:
  let offlinePage = response.get()
  echo "Received offline users page"
  discard httpResponse.statusCode
[inline-code-end]

---