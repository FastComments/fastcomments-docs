## Parámetros

| Nombre | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| skip | number | No |  |

## Respuesta

Devuelve: [`GetModerators200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerators200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getModerators'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-12345-prod';
const moderatorsPage1: GetModerators200Response = await getModerators(tenantId);
const moderatorsPage2: GetModerators200Response = await getModerators(tenantId, 50);
[inline-code-end]

---