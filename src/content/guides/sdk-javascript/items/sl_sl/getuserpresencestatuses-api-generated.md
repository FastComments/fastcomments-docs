## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlIdWS | string | Da |  |
| userIds | string | Da |  |

## Odziv

Vrne: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatuses200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getUserPresenceStatuses'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const urlIdWS: string = 'wss://comments.fastsite.com/ws/tenant_42';
const userIds: string = 'user_9012,user_3478';
const includePresenceMetadata: boolean | undefined = true; // primer neobveznega parametra
const presenceStatuses: GetUserPresenceStatuses200Response = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
[inline-code-end]

---