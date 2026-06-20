Пакетна інформація про користувачів для орендаря. За заданими userIds повертає інформацію для відображення з User / SSOUser.
Використовується віджетом коментарів для збагачення користувачів, які щойно з'явилися через подію присутності.
Без контексту сторінки: приватність застосовується однаково (приватні профілі приховуються).

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| ids | string | Ні |  |

## Відповідь

Повертає: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад використання getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user-42,user-87")
if response.isSome:
  let usersInfo = response.get()
  echo "Retrieved users info:", usersInfo
else:
  echo "No users info returned. HTTP status:", httpResponse.status
[inline-code-end]

---