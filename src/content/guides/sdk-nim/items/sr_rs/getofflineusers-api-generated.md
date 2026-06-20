Прошли коментатори на страници који тренутно нису онлајн. Сортирано по displayName.
Користите ово након што исцрпите /users/online да бисте приказали одељак "Чланови".
Курсорска пагинација по commenterName: сервер прелази делимични индекс {tenantId, urlId, commenterName}
од afterName унапред помоћу $gt, без трошкова $skip.

## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| afterName | string | Не |  |
| afterUserId | string | Не |  |

## Одговор

Враћа: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Пример

[inline-code-attrs-start title = 'getOfflineUsers пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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