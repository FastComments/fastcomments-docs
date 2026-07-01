## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Réponse

Renvoie : [`ResetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationCountResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple resetUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoResetCount() {
  const tenantId: string = "acme-corp-tenant";
  const sso: string = "sso-user-9876";

  // Appeler avec le paramètre sso optionnel
  const resultWithSso: ResetUserNotificationCountResponse = await resetUserNotificationCount(tenantId, sso);

  // Appeler sans le paramètre sso optionnel
  const resultWithoutSso: ResetUserNotificationCountResponse = await resetUserNotificationCount(tenantId);

  console.log(resultWithSso, resultWithoutSso);
}
[inline-code-end]

---