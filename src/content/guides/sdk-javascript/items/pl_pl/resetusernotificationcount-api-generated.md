## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Odpowiedź

Zwraca: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład resetUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoReset() {
  const tenantId: string = "tenant_12345";
  const ssoToken: string = "sso_abcdef123456";

  const resultWithSso: ResetUserNotificationsResponse = await resetUserNotificationCount(tenantId, ssoToken);
  const resultWithoutSso: ResetUserNotificationsResponse = await resetUserNotificationCount(tenantId);
}

demoReset();
[inline-code-end]