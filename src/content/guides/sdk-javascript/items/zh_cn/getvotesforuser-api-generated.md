## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 响应

返回: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetVotesForUserResponse.ts)

## 示例

[inline-code-attrs-start title = 'getVotesForUser 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "northwind-io";
const urlId: string = "blog/2025/06/typescript-updates";
const userId: string = "u_72f9b1c4";
const anonUserId: string = "anon_e4d2a9";

const votesForRegistered: GetVotesForUserResponse = await getVotesForUser(tenantId, urlId, userId);
const votesForAnonymous: GetVotesForUserResponse = await getVotesForUser(tenantId, urlId, undefined, anonUserId);
[inline-code-end]