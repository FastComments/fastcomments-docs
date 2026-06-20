## Parametri

| Name | Type | Obvezno | Opis |
|------|------|---------|------|
| userId | string | Ne |  |
| direction | SortDirections | Ne |  |
| repliesToUserId | string | Ne |  |
| page | float64 | Ne |  |
| includei10n | bool | Ne |  |
| locale | string | Ne |  |
| isCrawler | bool | Ne |  |

## Odgovor

Vrne: [`Option[GetCommentsForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_for_user_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getCommentsForUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---