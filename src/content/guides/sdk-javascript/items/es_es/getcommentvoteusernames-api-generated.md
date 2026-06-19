## Parámetros

| Nombre | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| dir | number | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentVoteUserNamesSuccessResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getCommentVoteUserNames'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-7f9c';
const commentId: string = 'bcd12345-6789-4ef0-9abc-0d1e2f3a4b5c';
const dir: number = 1;
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoPayload.signature';

const resultWithoutSso: GetCommentVoteUserNamesSuccessResponse = await getCommentVoteUserNames(tenantId, commentId, dir);
const resultWithSso: GetCommentVoteUserNamesSuccessResponse = await getCommentVoteUserNames(tenantId, commentId, dir, ssoToken);
[inline-code-end]

---