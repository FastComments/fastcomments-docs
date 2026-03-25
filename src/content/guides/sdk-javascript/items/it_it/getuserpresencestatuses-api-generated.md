---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| urlIdWS | string | Sì |  |
| userIds | string | Sì |  |

## Risposta

Restituisce: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatuses200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getUserPresenceStatuses'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3a2b';
const urlIdWS: string = 'articles/2026/03/25/fastcomments-integration';
const maybeUserIds: string | undefined = 'user_123,user_456'; // sorgente opzionale
const userIds: string = maybeUserIds ?? 'user_123';
const presence: GetUserPresenceStatuses200Response = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
[inline-code-end]

---