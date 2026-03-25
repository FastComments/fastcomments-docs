---
## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| afterId | string | Nej |  |
| afterCreatedAt | number | Nej |  |
| unreadOnly | boolean | Nej |  |
| dmOnly | boolean | Nej |  |
| noDm | boolean | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotifications200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på resetUserNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_prod_4a9f12";
const afterId: string = "notification_87213";
const afterCreatedAt: number = Math.floor(Date.now() / 1000) - 3600;
const unreadOnly: boolean = true;
const dmOnly: boolean = false;
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.payload";
const result: ResetUserNotifications200Response = await resetUserNotifications(tenantId, afterId, afterCreatedAt, unreadOnly, dmOnly, undefined, sso);
[inline-code-end]

---