Претходни коментари на страници који ТРЕНУТНО нису на мрежи. Сортирани по displayName.
Користите ово након што исцрпите /users/online за приказ секције „Чланови".
Пагинација помоћу курсора по commenterName: сервер пролази кроз парцијални {tenantId, urlId, commenterName}
индекс од afterName напред преко $gt, без трошка $skip.

## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| options | GetOfflineUsersOptions | Не |  |

## Одговор

Враћа: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getOfflineUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (offlineResp, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetOfflineUsersOptions()
)
if offlineResp.isSome:
  let offline = offlineResp.get()
  echo offline)
[inline-code-end]