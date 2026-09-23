## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| broadcastId | string | Da |  |
| commentData | CommentData | Da |  |
| sessionId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SaveCommentsResponseWithPresence.ts)

## Primer

[inline-code-attrs-start title = 'Primer createCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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