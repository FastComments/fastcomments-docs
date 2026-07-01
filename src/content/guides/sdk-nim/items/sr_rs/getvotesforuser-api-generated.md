## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetVotesForUserOptions | No |  |

## Одговор

Враћа: [`Option[GetVotesForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes_for_user_response.nim)

## Пример

[inline-code-attrs-start title = 'Primer getVotesForUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getVotesForUser(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetVotesForUserOptions()
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]