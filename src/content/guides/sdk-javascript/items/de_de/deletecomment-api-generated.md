## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| contextUserId | string | Nein |  |
| isLive | boolean | Nein |  |

## Antwort

Rückgabe: [`DeleteCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'deleteComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_01';
  const commentId: string = 'comment_abc123';
  const contextUserId: string = 'user_42';
  const isLive: boolean = false;

  const deleteResult: DeleteCommentResponse = await deleteComment(tenantId, commentId, contextUserId, isLive);
  const simpleResult: DeleteCommentResponse = await deleteComment(tenantId, commentId);
})();
[inline-code-end]