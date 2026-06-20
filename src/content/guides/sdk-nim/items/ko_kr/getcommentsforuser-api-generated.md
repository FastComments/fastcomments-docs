## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| userId | string | 아니오 |  |
| direction | SortDirections | 아니오 |  |
| repliesToUserId | string | 아니오 |  |
| page | float64 | 아니오 |  |
| includei10n | bool | 아니오 |  |
| locale | string | 아니오 |  |
| isCrawler | bool | 아니오 |  |

## 응답

반환: [`Option[GetCommentsForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_for_user_response.nim)

## 예제

[inline-code-attrs-start title = 'getCommentsForUser 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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