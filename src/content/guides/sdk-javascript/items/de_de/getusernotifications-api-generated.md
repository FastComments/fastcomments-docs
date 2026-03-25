## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| pageSize | number | Nein |  |
| afterId | string | Nein |  |
| includeContext | boolean | Nein |  |
| afterCreatedAt | number | Nein |  |
| unreadOnly | boolean | Nein |  |
| dmOnly | boolean | Nein |  |
| noDm | boolean | Nein |  |
| includeTranslations | boolean | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotifications200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getUserNotifications Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b1c';
const pageSize: number = 25;
const afterId: string = 'notif_b2f9e4';
const includeContext: boolean = true;
const afterCreatedAt: number = Date.now() - 24 * 60 * 60 * 1000;
const unreadOnly: boolean = true;
const dmOnly: boolean = false;
const noDm: boolean = false;
const includeTranslations: boolean = true;
const sso: string = 'sso_tok_user_9f8d7c';
const response: GetUserNotifications200Response = await getUserNotifications(
  tenantId,
  pageSize,
  afterId,
  includeContext,
  afterCreatedAt,
  unreadOnly,
  dmOnly,
  noDm,
  includeTranslations,
  sso
);
[inline-code-end]

---