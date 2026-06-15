---
## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| broadcastId | string | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`PinComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinComment200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de unPinComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f3b2c1a";
const commentId: string = "comment_4d2e8a7f";
const broadcastId: string = "broadcast_live_2026_06_15_18";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.invalid-signature";
const response: PinComment200Response = await unPinComment(tenantId, commentId, broadcastId, sso);
console.log(response);
[inline-code-end]

---