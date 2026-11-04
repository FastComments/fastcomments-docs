页面上当前不在线的历史评论者。按 displayName 排序。
在用尽 /users/online 之后，使用此接口渲染“成员”部分。
基于 commenterName 的游标分页：服务器通过 $gt 从 afterName 开始向前遍历部分索引 {tenantId, urlId, commenterName}，没有 $skip 的开销。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| options | GetOfflineUsersOptions | 否 |  |

## 响应

返回：[`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## 示例

[inline-code-attrs-start title = 'getOfflineUsers 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (offlineResp, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetOfflineUsersOptions()
)
if offlineResp.isSome:
  let offline = offlineResp.get()
  echo offline)
[inline-code-end]