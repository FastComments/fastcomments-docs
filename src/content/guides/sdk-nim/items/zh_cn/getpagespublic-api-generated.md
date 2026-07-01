列出租户的页面。由 FChat 桌面客户端用于填充其房间列表。  
需要在每个页面的解析自定义配置中将 `enableFChat` 设置为 true。  
需要 SSO 的页面会根据请求用户的组访问权限进行过滤。

## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|------|
| tenantId | string | Yes |  |
| options | GetPagesPublicOptions | No |  |

## 响应

返回：[`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## 示例

[inline-code-attrs-start title = 'getPagesPublic 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]