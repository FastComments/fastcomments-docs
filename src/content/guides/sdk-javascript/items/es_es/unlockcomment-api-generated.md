## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| broadcastId | string | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de unLockComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3d2b';
const commentId: string = 'cmt_5e8a1d';
const broadcastId: string = 'bcast_4f2b7a';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1aWQiOiJ1c2VyX2QxMjMifQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const result: APIEmptyResponse = await unLockComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]

---