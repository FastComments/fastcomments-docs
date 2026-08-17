## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |

## Одговор

Враћа: [`Option[GetVotesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes_response.nim)

## Пример

[inline-code-attrs-start title = 'getVotes Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optVotes, httpResp) = client.getVotes(tenantId = "my-tenant-123", urlId = "news/article-42")
if optVotes.isSome:
  let votes = optVotes.get()
  discard votes
[inline-code-end]