## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| userId | string | Hayır |  |
| direction | SortDirections | Hayır |  |
| repliesToUserId | string | Hayır |  |
| page | float64 | Hayır |  |
| includei10n | bool | Hayır |  |
| locale | string | Hayır |  |
| isCrawler | bool | Hayır |  |

## Yanıt

Döndürür: [`Option[GetCommentsForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_for_user_response.nim)

## Örnek

[inline-code-attrs-start title = 'getCommentsForUser Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentsForUser(
  userId = "user-8421",
  direction = SortDirections.Newest,
  repliesToUserId = "",
  page = 1.0,
  includei10n = true,
  locale = "en-US",
  isCrawler = false
)

if response.isSome:
  let comments = response.get()
  discard comments
[inline-code-end]