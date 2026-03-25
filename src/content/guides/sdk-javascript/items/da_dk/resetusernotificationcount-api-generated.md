## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotifications200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på resetUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_9f3b2c4a";
  const ssoToken: string | undefined = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9._sample_payload_.signature";
  const responseWithSSO: ResetUserNotifications200Response = await resetUserNotificationCount(tenantId, ssoToken);
  const responseWithoutSSO: ResetUserNotifications200Response = await resetUserNotificationCount(tenantId);
  console.log(responseWithSSO, responseWithoutSSO);
})();
[inline-code-end]

---