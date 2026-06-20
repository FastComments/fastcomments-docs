## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| userId | string | 任意 |  |
| direction | SortDirections | 任意 |  |
| repliesToUserId | string | 任意 |  |
| page | float64 | 任意 |  |
| includei10n | bool | 任意 |  |
| locale | string | 任意 |  |
| isCrawler | bool | 任意 |  |

## レスポンス

戻り値: [`Option[GetCommentsForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_for_user_response.nim)

## 例

[inline-code-attrs-start title = 'getCommentsForUser の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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