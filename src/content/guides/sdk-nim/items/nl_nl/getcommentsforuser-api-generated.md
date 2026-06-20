## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| userId | string | Nee |  |
| direction | SortDirections | Nee |  |
| repliesToUserId | string | Nee |  |
| page | float64 | Nee |  |
| includei10n | bool | Nee |  |
| locale | string | Nee |  |
| isCrawler | bool | Nee |  |

## Respons

Geeft terug: [`Option[GetCommentsForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_for_user_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getCommentsForUser Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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