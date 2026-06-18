req
tenantId
urlId
userIdWS

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| userIdWS | string | Sí |  |
| startTime | number | Sí |  |
| endTime | number | No |  |

## Respuesta

Devuelve: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLog200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getEventLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3a2b';
const urlId: string = 'news/2026/06/fastcomments-release';
const userIdWS: string = 'ws_user_48291';
const startTime: number = Date.now() - 86_400_000;
const endTime: number = Date.now();
const result: GetEventLog200Response = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
[inline-code-end]

---