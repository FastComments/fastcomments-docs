## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 响应

返回: [`Option[GetV1PageLikes]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_v1_page_likes.nim)

## 示例

[inline-code-attrs-start title = 'getV1PageLikes 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getV1PageLikes(tenantId = "my-tenant-123", urlId = "news/how-to-train-your-dragon")
if response.isSome:
  let pageLikes = response.get()
  echo "Fetched page likes for url:", "news/how-to-train-your-dragon"
else:
  echo "No likes returned for url:", "news/how-to-train-your-dragon"
[inline-code-end]