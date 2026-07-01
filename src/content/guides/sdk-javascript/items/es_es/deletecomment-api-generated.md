## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| contextUserId | string | No |  |
| isLive | boolean | No |  |

## Respuesta

Devuelve: [`DeleteCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo deleteComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---