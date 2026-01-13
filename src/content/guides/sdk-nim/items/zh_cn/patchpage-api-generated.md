## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| updateAPIPageData | UpdateAPIPageData | 否 |  |

## 响应

返回：[`Option[PatchPageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_page_api_response.nim)

## 示例

[inline-code-attrs-start title = 'patchPage 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateData = UpdateAPIPageData(
  title = "Breaking: Major Event Update",
  urlId = "news/major-event-update",
  visible = true,
  tags = @["breaking", "headline"],
  sortOrder = 5
)

let (response, httpResponse) = client.patchPage(
  tenantId = "my-tenant-123",
  id = "news/major-event-update",
  updateAPIPageData = updateData
)

if response.isSome:
  let page = response.get()
  discard page
[inline-code-end]

---