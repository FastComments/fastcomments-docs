## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 回應

回傳: [`Option[GetV1PageLikes]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_v1_page_likes.nim)

## 範例

[inline-code-attrs-start title = 'getV1PageLikes 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getV1PageLikes(tenantId = "my-tenant-123", urlId = "news/article-title")
if response.isSome:
  let pageLikes = response.get()
  # 根據需要使用 pageLikes
[inline-code-end]

---