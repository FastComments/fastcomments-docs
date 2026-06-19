---
## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de resetUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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