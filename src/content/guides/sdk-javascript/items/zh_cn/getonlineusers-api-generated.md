---
当前页面的在线查看者：当前其 websocket 会话已订阅该页面的人。
返回 anonCount + totalCount（房间范围的订阅者，包括我们不列举的匿名查看者）。

## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| afterName | string | 否 |  |
| afterUserId | string | 否 |  |

## 响应

返回：[`GetOnlineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsers200Response.ts)

## 示例

[inline-code-attrs-start title = 'getOnlineUsers 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_14f9c3';
const urlId: string = 'article_20250615';
const afterName: string = 'marie.curie';
const afterUserId: string = 'u_92b7';
const result: GetOnlineUsers200Response = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---