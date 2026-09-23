## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| setCommentTextParams | SetCommentTextParams | Sí |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentTextResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'postSetCommentText Ejemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_9876";
const setCommentTextParams: SetCommentTextParams = {
  text: "Revised comment after user edit.",
  format: "plain"
};
const broadcastId: string = "broadcast_2023_09";

const response: SetCommentTextResponse = await postSetCommentText(
  tenantId,
  commentId,
  setCommentTextParams,
  broadcastId
);

console.log(response);
[inline-code-end]