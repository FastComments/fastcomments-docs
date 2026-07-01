## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`LockCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/LockCommentResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo lockComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const broadcastId: string = "brd_54321";

  // Con token SSO opcional
  const ssoToken: string = "user-abc123";
  const lockedWithSso: LockCommentResponse = await lockComment(tenantId, commentId, broadcastId, ssoToken);

  // Sin token SSO
  const lockedWithoutSso: LockCommentResponse = await lockComment(tenantId, commentId, broadcastId);
})();
[inline-code-end]

---