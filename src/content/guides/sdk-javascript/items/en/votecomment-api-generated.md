## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | Yes |  |
| voteBodyParams | VoteBodyParams | Yes |  |
| sessionId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/VoteComment200Response.ts)

## Example

[inline-code-attrs-start title = 'voteComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme';
const commentId: string = 'cmt_123456789';
const urlId: string = 'https://www.acme-news.com/articles/ai-update-2026';
const broadcastId: string = 'broadcast_live_20260520';
const voteBodyParams: VoteBodyParams = { value: 1, reason: 'Insightful contribution' };
const sessionId: string = 'sess_9b8f7a6c';
const sso: string = 'sso_token_eyJhbGciOiJIUzI1Ni';
const result: VoteComment200Response = await voteComment(tenantId, commentId, urlId, broadcastId, voteBodyParams, sessionId, sso);
[inline-code-end]
