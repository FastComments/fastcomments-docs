## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| editKey | string | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentText200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getCommentText'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42b7e9';
const commentId: string = 'cmt_9f3a2b';
const editKey: string = 'edk_3f1b7c9d';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoPayload.signature';

const result: GetCommentText200Response = await getCommentText(tenantId, commentId, editKey, sso);
[inline-code-end]

---