## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## 响应

返回: [`Option[DeletePageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_page_api_response.nim)

## 示例

[inline-code-attrs-start title = 'deletePage 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (deleteRespOpt, httpResp) = client.deletePage(tenantId = "my-tenant-123", id = "news/article-title")
if deleteRespOpt.isSome:
  let deleteResp = deleteRespOpt.get()
  discard deleteResp
discard httpResp
[inline-code-end]

---