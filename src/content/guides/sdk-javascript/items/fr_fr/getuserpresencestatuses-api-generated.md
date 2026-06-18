## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlIdWS | string | Oui |  |
| userIds | string | Oui |  |

## Réponse

Renvoie : [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatuses200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getUserPresenceStatuses'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const urlIdWS: string = 'wss://comments.fastsite.com/ws/tenant_42';
const userIds: string = 'user_9012,user_3478';
const includePresenceMetadata: boolean | undefined = true; // exemple de paramètre optionnel
const presenceStatuses: GetUserPresenceStatuses200Response = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
[inline-code-end]