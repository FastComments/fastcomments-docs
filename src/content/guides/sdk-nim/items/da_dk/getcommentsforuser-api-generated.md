## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| userId | string | Nej |  |
| direction | SortDirections | Nej |  |
| repliesToUserId | string | Nej |  |
| page | float64 | Nej |  |
| includei10n | bool | Nej |  |
| locale | string | Nej |  |
| isCrawler | bool | Nej |  |

## Svar

Returnerer: [`Option[GetCommentsForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_for_user_response.nim)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getCommentsForUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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