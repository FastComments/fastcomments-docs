## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| textSearch | string | いいえ |  |
| byIPFromComment | string | いいえ |  |
| filter | string | いいえ |  |
| searchFilters | string | いいえ |  |
| demo | bool | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`Option[ModerationAPICountCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_count_comments_response.nim)

## 例

[inline-code-attrs-start title = 'getCount の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCount(
  textSearch = "climate change",
  byIPFromComment = "203.0.113.5",
  filter = "status:approved",
  searchFilters = "author:john.doe@example.com;tag:opinion",
  demo = false,
  sso = "sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
)
if response.isSome:
  let countResp = response.get()
  discard countResp
  echo "Count response received"
else:
  echo "No count data returned"
[inline-code-end]

---