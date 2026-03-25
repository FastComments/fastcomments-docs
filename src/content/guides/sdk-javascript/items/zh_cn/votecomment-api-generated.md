## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| urlId | string | 是 |  |
| broadcastId | string | 是 |  |
| voteBodyParams | VoteBodyParams | 是 |  |
| sessionId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteComment200Response.ts)

## 示例

[inline-code-attrs-start title = 'voteComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b7c';
const commentId: string = 'cmt_42f3a1';
const urlId: string = 'articles/ai-trends-2026';
const broadcastId: string = 'web';
const voteBodyParams: VoteBodyParams = { vote: 1, reason: 'Insightful and on-topic' };
const sessionId: string = 'sess_6d2b4c9e';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const result: VoteComment200Response = await voteComment(tenantId, commentId, urlId, broadcastId, voteBodyParams, sessionId, sso);
[inline-code-end]

---