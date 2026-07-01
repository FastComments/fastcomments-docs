---
页面上过去的评论者（当前不在线）。按 displayName 排序。  
在使用完 /users/online 后使用，以渲染“成员”部分。  
在 commenterName 上进行光标分页：服务器遍历部分 {tenantId, urlId, commenterName}，从 afterName 开始向前使用 $gt 索引，无需 $skip 开销。

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Response

返回：[`GetOfflineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsersResponse.ts)

## Example

[inline-code-attrs-start title = 'getOfflineUsers 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOfflineUsers(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const urlId: string = "thread_9876";
    const afterName: string = "Jane Smith";
    const afterUserId: string = "user_7f9b3c";

    const offlineUsers: GetOfflineUsersResponse = await getOfflineUsers(
        tenantId,
        urlId,
        afterName,
        afterUserId
    );

    console.log(offlineUsers);
}
[inline-code-end]

---