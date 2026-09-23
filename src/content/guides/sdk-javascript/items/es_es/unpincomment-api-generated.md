## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeCommentPinStatusResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de unPinComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "comment-20230915-001";
  const broadcastId: string = "broadcast-2023-09-15";
  const ssoToken: string = "sso-abc123";

  const result: ChangeCommentPinStatusResponse = await unPinComment(tenantId, commentId, broadcastId, ssoToken);
  const resultWithoutSso: ChangeCommentPinStatusResponse = await unPinComment(tenantId, commentId, broadcastId);
})();
[inline-code-end]