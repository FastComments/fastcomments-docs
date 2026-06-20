## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| userId | string | לא |  |
| direction | SortDirections | לא |  |
| repliesToUserId | string | לא |  |
| page | float64 | לא |  |
| includei10n | bool | לא |  |
| locale | string | לא |  |
| isCrawler | bool | לא |  |

## תגובה

מחזיר: [`Option[GetCommentsForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_for_user_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCommentsForUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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