## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| broadcastId | string | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeCommentPinStatusResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de pinComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoPinComment() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const broadcastId: string = "brd_54321";

  const resultWithoutSso: ChangeCommentPinStatusResponse = await pinComment(tenantId, commentId, broadcastId);
  const ssoToken: string = "sso_abcde12345";
  const resultWithSso: ChangeCommentPinStatusResponse = await pinComment(tenantId, commentId, broadcastId, ssoToken);
}

demoPinComment();
[inline-code-end]