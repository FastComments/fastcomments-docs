## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | Yes |  |
| commentData | CommentData | Yes |  |
| sessionId | string | No |  |
| sso | string | No |  |

## Response

Returns: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'createCommentPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_42north";
const urlId: string = "news/2026/05/20/live-event";
const broadcastId: string = "brdcst_7f3d9c";
const commentData: CommentData = {
  body: "Great analysis — appreciated the insights on market trends.",
  userMentions: [{ userId: "user_102", displayName: "Jamie" }],
  hashtags: ["market", "analysis"]
} as CommentData;
const sessionId: string | undefined = "sess_ab12cd34";
const sso: string | undefined = "sso-token-98b7";
const result: CreateCommentPublic200Response = await createCommentPublic(tenantId, urlId, broadcastId, commentData, sessionId, sso);
[inline-code-end]
