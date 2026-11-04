租户的用户批量信息。给定 userIds，从 User / SSOUser 返回显示信息。
由评论组件用于丰富刚通过在线状态事件出现的用户。
无页面上下文：隐私保护统一执行（私密个人资料会被遮蔽）。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| ids | string | 否 |  |

## 响应

返回：[`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## 示例

[inline-code-attrs-start title = 'getUsersInfo 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]