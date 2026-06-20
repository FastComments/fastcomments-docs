## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| userId | string | 否 |  |
| direction | SortDirections | 否 |  |
| repliesToUserId | string | 否 |  |
| page | float64 | 否 |  |
| includei10n | bool | 否 |  |
| locale | string | 否 |  |
| isCrawler | bool | 否 |  |

## 响应

返回: [`Option[GetCommentsForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_for_user_response.nim)

## 示例

[inline-code-attrs-start title = 'getCommentsForUser 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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