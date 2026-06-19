## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlIdWS | string | Sí |  |
| userIds | string | Sí |  |

## Respuesta

Devuelve: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatusesResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getUserPresenceStatuses'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acct_7c9b21';
const urlIdWS: string = 'wss://ws.fastcomments.com/presence/room-231';
const userIds: string = 'user_102,user_203';
const presence: GetUserPresenceStatusesResponse = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
const firstStatus: APIStatus | undefined = (presence as unknown as { statuses?: APIStatus[] }).statuses?.[0];
[inline-code-end]

---