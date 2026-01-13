## 参数

| 名称 | 类型 | 是否必需 | 说明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |

## 响应

返回：[`Option[GetPageByURLIdAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_page_by_urlid_api_response.nim)

## 示例

[inline-code-attrs-start title = 'getPageByURLId 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPageByURLId(tenantId = "my-tenant-123", urlId = "news/article-title")
if response.isSome:
  let page = response.get()
  echo "Page ID: ", page.id
  echo "Title: ", page.title
  echo "URL: ", page.url
  echo "Published: ", $page.published
  echo "Tags: ", $page.tags
else:
  echo "No page found. HTTP status: ", httpResponse.statusCode
[inline-code-end]

---