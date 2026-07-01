## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| contextUserId | string | Hayır |  |
| isLive | boolean | Hayır |  |

## Yanıt

Döndürür: [`DeleteCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentResponse.ts)

## Örnek

[inline-code-attrs-start title = 'deleteComment Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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