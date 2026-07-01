## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| options | GetSearchPagesOptions | 否 |  |

## 响应

返回：[`Option[ModerationPageSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_page_search_response.nim)

## 示例

[inline-code-attrs-start title = 'getSearchPages 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (searchResp, httpResp) = client.getSearchPages(
  tenantId = "my-tenant-123",
  options = GetSearchPagesOptions(
    page: 1,
    pageSize: 20,
    query: "spam"
  )
)

if searchResp.isSome:
  let resp = searchResp.get()
[inline-code-end]

---