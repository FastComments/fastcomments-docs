## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 响应

返回: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUser200Response.ts)

## 示例

[inline-code-attrs-start title = 'getVotesForUser 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const urlId: string = 'news/2026/01/12/product-launch';
const userId: string = 'user_9c3f2b';
const anonUserId: string = 'anon_d4e7a1';

const votesForUser: GetVotesForUser200Response = await getVotesForUser(tenantId, urlId, userId);
const votesForAnon: GetVotesForUser200Response = await getVotesForUser(tenantId, urlId, undefined, anonUserId);
[inline-code-end]

---