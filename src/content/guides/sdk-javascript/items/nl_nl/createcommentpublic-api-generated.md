## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| broadcastId | string | Ja |  |
| commentData | CommentData | Ja |  |
| sessionId | string | Nee |  |
| sso | string | Nee |  |

## Respons

Returns: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveCommentsResponseWithPresence.ts)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld createCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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