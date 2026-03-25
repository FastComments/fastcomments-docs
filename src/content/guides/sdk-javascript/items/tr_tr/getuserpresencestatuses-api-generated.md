## Parametreler

| Name | Type | Zorunlu | Açıklama |
|------|------|---------|----------|
| tenantId | string | Evet |  |
| urlIdWS | string | Evet |  |
| userIds | string | Evet |  |

## Yanıt

Döndürür: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatuses200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getUserPresenceStatuses Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3a2b';
const urlIdWS: string = 'articles/2026/03/25/fastcomments-integration';
const maybeUserIds: string | undefined = 'user_123,user_456'; // isteğe bağlı kaynak
const userIds: string = maybeUserIds ?? 'user_123';
const presence: GetUserPresenceStatuses200Response = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
[inline-code-end]

---