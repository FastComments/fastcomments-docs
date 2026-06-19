## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlIdWS | string | Ja |  |
| userIds | string | Ja |  |

## Antwort

Gibt zurück: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatusesResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getUserPresenceStatuses'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acct_7c9b21';
const urlIdWS: string = 'wss://ws.fastcomments.com/presence/room-231';
const userIds: string = 'user_102,user_203';
const presence: GetUserPresenceStatusesResponse = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
const firstStatus: APIStatus | undefined = (presence as unknown as { statuses?: APIStatus[] }).statuses?.[0];
[inline-code-end]