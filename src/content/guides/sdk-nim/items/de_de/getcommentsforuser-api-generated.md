## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| userId | string | Nein |  |
| direction | SortDirections | Nein |  |
| repliesToUserId | string | Nein |  |
| page | float64 | Nein |  |
| includei10n | bool | Nein |  |
| locale | string | Nein |  |
| isCrawler | bool | Nein |  |

## Antwort

Gibt zurück: [`Option[GetCommentsForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_for_user_response.nim)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getCommentsForUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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