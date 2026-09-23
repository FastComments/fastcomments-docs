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

Returns: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveCommentsResponseWithPresence.ts)

## Example

[inline-code-attrs-start title = 'createCommentPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant-456";
  const urlId: string = "url-789";
  const broadcastId: string = "broadcast-101112";
  const commentData: CommentData = { text: "This is a comment", userId: "user-123" };
  const sessionId: string = "session-131415";
  const sso: string = "sso-token-xyz";
  const result: SaveCommentsResponseWithPresence = await createCommentPublic(tenantId, urlId, broadcastId, commentData, sessionId, sso);
  const resultNoOpt: SaveCommentsResponseWithPresence = await createCommentPublic(tenantId, urlId, broadcastId, commentData);
}
[inline-code-end]
