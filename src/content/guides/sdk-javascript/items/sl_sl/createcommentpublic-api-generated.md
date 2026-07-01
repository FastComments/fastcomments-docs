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

Vrne: [`CreateCommentPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateCommentPublicResponse.ts)

## Primer

[inline-code-attrs-start title = 'createCommentPublic Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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