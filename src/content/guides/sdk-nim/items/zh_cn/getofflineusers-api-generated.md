---
页面上之前发表评论但当前不在线的用户。按 displayName 排序。
在用尽 /users/online 后使用此方法以渲染“成员”部分。
对 commenterName 进行游标分页：服务器会遍历部分 {tenantId, urlId, commenterName}
索引从 afterName 向前通过 $gt， 无 $skip 成本。

## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| afterName | string | 否 |  |
| afterUserId | string | 否 |  |

## 响应

返回：[`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## 示例

[inline-code-attrs-start title = 'getOfflineUsers 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-how-to-code",
  afterName = "",
  afterUserId = ""
)

if response.isSome:
  let offlinePage = response.get()
  echo "Received offline users page"
  discard httpResponse.statusCode
[inline-code-end]

---