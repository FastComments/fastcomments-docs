## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |

## Respuesta

Devuelve: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModeratorResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchModerator() {
  const tenantId: string = "c9f1e2b3-4d5a-6f78-90ab-cdef12345678";
  const moderatorId: string = "mod-987654";
  const response: GetModeratorResponse = await getModerator(tenantId, moderatorId);
  const isActive: boolean | undefined = response.moderator?.isActive;
  const statusCode: number | undefined = response.status?.code;
}
[inline-code-end]

---