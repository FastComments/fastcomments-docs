页面上曾评论但当前不在线的用户。按 displayName 排序。
在用尽 /users/online 之后使用此项来呈现 "成员" 部分。
基于 commenterName 的游标分页：服务器遍历部分 {tenantId, urlId, commenterName}
从 afterName 之后通过 $gt 向前索引，无 $skip 成本。

## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| afterName | string | 否 |  |
| afterUserId | string | 否 |  |

## 响应

返回: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## 示例

[inline-code-attrs-start title = 'getOfflineUsers 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f4b2a6c';
const urlId: string = 'articles/product-launch-2025';

const offlinePageFirst: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId);

const afterName: string = 'samantha.r';
const afterUserId: string = 'user_7d3a21f9';
const offlinePageNext: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---