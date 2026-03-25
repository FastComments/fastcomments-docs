## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| contextUserId | string | No |  |
| isLive | boolean | No |  |

## Respuesta

Devuelve: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteComment200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7b3f2a';
const commentId: string = 'cmt_8a1f4d2b';
const contextUserId: string = 'user_102';
const isLive: boolean = true;
const result: DeleteComment200Response = await deleteComment(tenantId, commentId, contextUserId, isLive);
[inline-code-end]

---