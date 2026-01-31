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
const tenantId: string = 'acme-tenant-42';
const commentId: string = 'cmt_20260109_824';
const urlId: string = 'news-article-2026-01-09';
const broadcastId: string = 'broadcast_7f3a9c';
const voteBodyParams: VoteBodyParams = { vote: 'up', reason: 'Insightful', weight: 1 };
const sessionId: string | undefined = 'sess_9b8c7';
const sso: string | undefined = 'sso_jwt_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const result: VoteComment200Response = await voteComment(tenantId, commentId, urlId, broadcastId, voteBodyParams, sessionId, sso);
[inline-code-end]
