---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| textSearch | string | 否 |  |
| byIPFromComment | string | 否 |  |
| filter | string | 否 |  |
| searchFilters | string | 否 |  |
| demo | bool | 否 |  |
| sso | string | 否 |  |

## 返回

返回： [`Option[ModerationAPICountCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_count_comments_response.nim)

## 示例

[inline-code-attrs-start title = 'getCount 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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