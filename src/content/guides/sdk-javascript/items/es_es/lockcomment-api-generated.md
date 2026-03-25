## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| commentId | string | Sí |  |
| broadcastId | string | Sí |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`LockComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/LockComment200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de lockComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_prod_8f3a2b";
const commentId: string = "cmt_5d7e9a92";
const broadcastId: string = "broadcast_2026_03_25_1400";
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature";
const resultWithSso: LockComment200Response = await lockComment(tenantId, commentId, broadcastId, ssoToken);
const resultWithoutSso: LockComment200Response = await lockComment(tenantId, commentId, broadcastId);
[inline-code-end]

---