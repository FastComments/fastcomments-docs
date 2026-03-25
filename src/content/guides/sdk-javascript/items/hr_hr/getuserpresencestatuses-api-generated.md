## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlIdWS | string | Da |  |
| userIds | string | Da |  |

## Odgovor

Vraća: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatuses200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getUserPresenceStatuses'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3a2b';
const urlIdWS: string = 'articles/2026/03/25/fastcomments-integration';
const maybeUserIds: string | undefined = 'user_123,user_456'; // neobavezni izvor
const userIds: string = maybeUserIds ?? 'user_123';
const presence: GetUserPresenceStatuses200Response = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
[inline-code-end]

---