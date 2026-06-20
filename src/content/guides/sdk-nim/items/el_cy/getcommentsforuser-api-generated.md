## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| userId | string | Όχι |  |
| direction | SortDirections | Όχι |  |
| repliesToUserId | string | Όχι |  |
| page | float64 | Όχι |  |
| includei10n | bool | Όχι |  |
| locale | string | Όχι |  |
| isCrawler | bool | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[GetCommentsForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_for_user_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentsForUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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