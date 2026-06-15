## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| afterId | string | Nee |  |
| afterCreatedAt | number | Nee |  |
| unreadOnly | boolean | Nee |  |
| dmOnly | boolean | Nee |  |
| noDm | boolean | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotifications200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'resetUserNotifications Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9b1f2";
const afterId: string = "notification_0001";
const afterCreatedAt: number = Date.now() - 60 * 60 * 1000; // een uur geleden
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