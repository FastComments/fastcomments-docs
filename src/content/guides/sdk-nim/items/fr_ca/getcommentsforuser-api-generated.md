## Paramètres

| Name | Type | Requis | Description |
|------|------|----------|-------------|
| userId | string | Non |  |
| direction | SortDirections | Non |  |
| repliesToUserId | string | Non |  |
| page | float64 | Non |  |
| includei10n | bool | Non |  |
| locale | string | Non |  |
| isCrawler | bool | Non |  |

## Réponse

Retourne : [`Option[GetCommentsForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_for_user_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentsForUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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