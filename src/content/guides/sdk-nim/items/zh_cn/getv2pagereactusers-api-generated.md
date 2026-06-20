---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| id | string | 否 |  |

## 响应

返回: [`Option[GetV2PageReactUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_v2_page_react_users_response.nim)

## 示例

[inline-code-attrs-start title = 'getV2PageReactUsers 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getV2PageReactUsers(tenantId = "my-tenant-123", urlId = "news/article-title", id = "")
if response.isSome:
  let usersResp = response.get()
  echo repr(usersResp)
else:
  echo "No page react users returned. HTTP response: ", repr(httpResponse)
[inline-code-end]

---