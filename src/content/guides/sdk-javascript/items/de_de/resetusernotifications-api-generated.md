## Parameter

| Name | Type | Erforderlich | Beschreibung |
|------|------|--------------|-------------|
| tenantId | string | Ja |  |
| afterId | string | Nein |  |
| afterCreatedAt | number | Nein |  |
| unreadOnly | boolean | Nein |  |
| dmOnly | boolean | Nein |  |
| noDm | boolean | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'resetUserNotifications Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-9f2b';
const afterId: string = 'notif_7c1a2b3';
const afterCreatedAt: number = Date.now() - 3 * 24 * 60 * 60 * 1000;
const unreadOnly: boolean = true;
const sso: string = 'sso:microsoft:84012';
const response: ResetUserNotificationsResponse = await resetUserNotifications(tenantId, afterId, afterCreatedAt, unreadOnly, undefined, undefined, sso);
[inline-code-end]

---