## Παράμετροι

| Name | Type | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlIdWS | string | Ναι |  |
| userIds | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatusesResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'getUserPresenceStatuses Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acct_7c9b21';
const urlIdWS: string = 'wss://ws.fastcomments.com/presence/room-231';
const userIds: string = 'user_102,user_203';
const presence: GetUserPresenceStatusesResponse = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
const firstStatus: APIStatus | undefined = (presence as unknown as { statuses?: APIStatus[] }).statuses?.[0];
[inline-code-end]