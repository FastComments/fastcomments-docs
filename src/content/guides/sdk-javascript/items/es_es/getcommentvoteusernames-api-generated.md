## Parámetros

| Name | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| dir | number | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentVoteUserNames200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getCommentVoteUserNames'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_4f2c1e';
  const commentId: string = 'cmt_9a7b3d';
  const dir: number = 1;
  const resultUpvotes: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dir);
  const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fakepayload.signature';
  const dirDown: number = -1;
  const resultDownvotes: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dirDown, sso);
  console.log(resultUpvotes, resultDownvotes);
})();
[inline-code-end]