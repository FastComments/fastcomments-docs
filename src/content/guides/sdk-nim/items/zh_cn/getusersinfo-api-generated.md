---
为租户批量获取用户信息。给定 userIds，返回来自 User / SSOUser 的显示信息。
由评论小部件使用，以丰富通过 presence 事件刚出现的用户信息。
无页面上下文：隐私统一强制（私人资料将被隐藏）。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | No |  |

## 响应

返回: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## 示例

[inline-code-attrs-start title = 'getUsersInfo 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user-42,user-87")
if response.isSome:
  let usersInfo = response.get()
  echo "Retrieved users info:", usersInfo
else:
  echo "No users info returned. HTTP status:", httpResponse.status
[inline-code-end]

---