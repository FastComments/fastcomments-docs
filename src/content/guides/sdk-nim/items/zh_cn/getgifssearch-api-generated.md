## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| search | string | 否 |  |
| locale | string | 否 |  |
| rating | string | 否 |  |
| page | float64 | 否 |  |

## 响应

返回： [`Option[GetGifsSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_gifs_search_response.nim)

## 示例

[inline-code-attrs-start title = 'getGifsSearch 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getGifsSearch(
  tenantId = "my-tenant-123",
  search = "funny cat",
  locale = "en-US",
  rating = "PG",
  page = 1.0
)

if response.isSome:
  let gifs = response.get()
  echo "Fetched GIFs response:", gifs
[inline-code-end]

---