## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| page | float64 | いいえ |  |
| count | float64 | いいえ |  |
| textSearch | string | いいえ |  |
| byIPFromComment | string | いいえ |  |
| filters | string | いいえ |  |
| searchFilters | string | いいえ |  |
| sorts | string | いいえ |  |
| demo | bool | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

返却値: [`Option[ModerationAPIGetCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comments_response.nim)

## 例

[inline-code-attrs-start title = 'getApiComments の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getApiComments(
  page = 1.0,
  count = 25.0,
  textSearch = "opinion on climate summit",
  byIPFromComment = "198.51.100.23",
  filters = "status:approved",
  searchFilters = "section:world",
  sorts = "-createdAt",
  demo = false,
  sso = "sso-user-982bf"
)

if response.isSome:
  let commentsResp = response.get()
  echo "Retrieved comments response"
else:
  echo "No comments returned, HTTP status: ", httpResponse.status
[inline-code-end]

---