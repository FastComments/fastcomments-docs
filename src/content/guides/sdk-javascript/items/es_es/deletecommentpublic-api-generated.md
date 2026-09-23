## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| broadcastId | string | Sí |  |
| editKey | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`PublicAPIDeleteCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicAPIDeleteCommentResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo deleteCommentPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_9876";
  const broadcastId: string = "brd_5555";
  const editKey: string = "edit_abc123";
  const sso: string = "sso_token_xyz";

  const resultWithoutOptional: PublicAPIDeleteCommentResponse = await deleteCommentPublic(tenantId, commentId, broadcastId);
  const resultWithOptional: PublicAPIDeleteCommentResponse = await deleteCommentPublic(tenantId, commentId, broadcastId, editKey, sso);
})();
[inline-code-end]

---