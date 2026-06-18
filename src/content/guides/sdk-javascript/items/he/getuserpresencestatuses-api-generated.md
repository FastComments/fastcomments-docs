## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlIdWS | string | כן |  |
| userIds | string | כן |  |

## תגובה

מחזיר: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserPresenceStatuses200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getUserPresenceStatuses'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const urlIdWS: string = 'wss://comments.fastsite.com/ws/tenant_42';
const userIds: string = 'user_9012,user_3478';
const includePresenceMetadata: boolean | undefined = true; // דוגמה לפרמטר אופציונלי
const presenceStatuses: GetUserPresenceStatuses200Response = await getUserPresenceStatuses(tenantId, urlIdWS, userIds);
[inline-code-end]

---