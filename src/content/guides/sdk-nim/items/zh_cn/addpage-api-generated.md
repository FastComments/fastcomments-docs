## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPIPageData | CreateAPIPageData | No |  |

## 响应

返回：[`Option[AddPageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_page_api_response.nim)

## 示例

[inline-code-attrs-start title = 'addPage 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let pageData = CreateAPIPageData(
  urlId = "news/article-2024",
  title = "Breaking News: Nim Takes Over",
  description = "An in-depth article about Nim's rise.",
  tags = @["nim", "programming", "news"]
)

let (addPageResp, httpResp) = client.addPage(
  tenantId = "my-tenant-123",
  createAPIPageData = pageData
)

if addPageResp.isSome:
  let resp = addPageResp.get()
  echo resp
[inline-code-end]