Пакетна інформація про користувачів для орендаря. За заданими userIds повертає інформацію для відображення з User / SSOUser. Використовується віджетом коментарів для збагачення користувачів, які лише‑що з’явилися через подію присутності. Без контексту сторінки: конфіденційність застосовується уніфіковано (приватні профілі маскуються).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | No |  |

## Response

Returns: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Example

[inline-code-attrs-start title = 'Приклад getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]