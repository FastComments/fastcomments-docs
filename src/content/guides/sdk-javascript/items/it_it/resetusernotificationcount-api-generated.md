## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| sso | string | No |  |

## Risposta

Restituisce: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di resetUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example(): Promise<void> {
  const tenantId: string = 'tenant_84f3b2';
  const resetResultNoSso: ResetUserNotificationsResponse = await resetUserNotificationCount(tenantId);

  const ssoToken: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
  const resetResultWithSso: ResetUserNotificationsResponse = await resetUserNotificationCount(tenantId, ssoToken);

  console.log(resetResultNoSso, resetResultWithSso);
}

example();
[inline-code-end]

---