## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| afterId | string | Nein |  |
| afterCreatedAt | number | Nein |  |
| unreadOnly | boolean | Nein |  |
| dmOnly | boolean | Nein |  |
| noDm | boolean | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotifications200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'resetUserNotifications Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9b1f2";
const afterId: string = "notification_0001";
const afterCreatedAt: number = Date.now() - 60 * 60 * 1000; // vor einer Stunde
const unreadOnly: boolean = true;
const dmOnly: boolean = false;
const noDm: boolean = false;
const sso: string = "sso_session_7f2d";
const result: ResetUserNotifications200Response = await resetUserNotifications(
  tenantId,
  afterId,
  afterCreatedAt,
  unreadOnly,
  dmOnly,
  noDm,
  sso
);
[inline-code-end]

---