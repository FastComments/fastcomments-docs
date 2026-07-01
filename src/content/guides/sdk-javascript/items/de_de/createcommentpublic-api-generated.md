## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|---------------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| broadcastId | string | Ja |  |
| commentData | CommentData | Ja |  |
| sessionId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`CreateCommentPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateCommentPublicResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'createCommentPublic Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-abc123";
  const urlId: string = "post-987654";
  const broadcastId: string = "bcast-001";
  const commentData: CommentData = {
    content: "I really enjoyed this article!"
  };
  const sessionId: string = "session-xyz789";
  const sso: string = "sso-token-456def";

  const response: CreateCommentPublicResponse = await createCommentPublic(
    tenantId,
    urlId,
    broadcastId,
    commentData,
    sessionId,
    sso
  );

  console.log(response);
})();
[inline-code-end]