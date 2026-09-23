## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlIdWS | string | Sí |  |
| userIds | string | Sí |  |

## Respuesta

Devuelve: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatusesResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getUserPresenceStatuses'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "my-tenant-42";
  const urlIdWS: string = "post-9876";
  const userIds: string = "alice,bob,carol";

  const presence: GetUserPresenceStatusesResponse = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
  console.log(presence);
})();
[inline-code-end]