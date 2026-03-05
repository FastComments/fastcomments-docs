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
const tenantId: string = "tenant_9f8d7c";
const commentId: string = "c_5a1b2c3d";
const urlId: string = "news/2026/01/clean-energy-policy";
const broadcastId: string = "broadcast_20260112_live";
const voteBodyParams: VoteBodyParams = { vote: "up", weight: 1 };
const sessionId: string | undefined = "session_xyz123";
const sso: string | undefined = "sso_jwt_token_eyJ";
const result: VoteComment200Response = await voteComment(tenantId, commentId, urlId, broadcastId, voteBodyParams, sessionId, sso);
[inline-code-end]
