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
const tenantId: string = 'tenant_news_42';
const commentId: string = 'cmt_5f8b9a2e';
const urlId: string = '/news/2025/11/22/election-results';
const broadcastId: string = 'brd_7f4a3';
const voteBodyParams: VoteBodyParams = { direction: 'up', weight: 1, reason: 'Helpful and on-topic' };
const sessionId: string = 'sess-9b8a7c6d1234';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.signedPayload';

const result: VoteComment200Response = await voteComment(tenantId, commentId, urlId, broadcastId, voteBodyParams, sessionId, sso);
[inline-code-end]
