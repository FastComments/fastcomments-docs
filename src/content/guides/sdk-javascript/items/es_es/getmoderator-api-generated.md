## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |

## Respuesta

Devuelve: [`GetModerator200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerator200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_eu_4f8d2b9e";
const maybeModeratorId: string | undefined = "mod_91c3b7a2"; // origen opcional (podría ser undefined)
const moderator: GetModerator200Response = await getModerator(tenantId, maybeModeratorId!);
[inline-code-end]