## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlIdWS | string | Evet |  |
| userIds | string | Evet |  |

## Yanıt

Döndürür: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatuses200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getUserPresenceStatuses Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const urlIdWS: string = 'wss://comments.fastsite.com/ws/tenant_42';
const userIds: string = 'user_9012,user_3478';
const includePresenceMetadata: boolean | undefined = true; // isteğe bağlı parametre örneği
const presenceStatuses: GetUserPresenceStatuses200Response = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
[inline-code-end]

---