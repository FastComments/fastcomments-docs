## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |

## 响应

返回：[`Option[GetPagesAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pages_api_response.nim)

## 示例

[inline-code-attrs-start title = 'getPages 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPages(tenantId = "news-site-456")
if response.isSome:
  let pages = response.get()
  echo "Received pages response: ", pages
else:
  echo "No pages returned. HTTP response: ", httpResponse
[inline-code-end]

---