## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Odgovor

Vraća: [`ResetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationCountResponse.ts)

## Primjer

[inline-code-attrs-start title = 'resetUserNotificationCount Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoResetCount() {
  const tenantId: string = "acme-corp-tenant";
  const sso: string = "sso-user-9876";

  // Call with optional sso parameter
  const resultWithSso: ResetUserNotificationCountResponse = await resetUserNotificationCount(tenantId, sso);

  // Call without optional sso parameter
  const resultWithoutSso: ResetUserNotificationCountResponse = await resetUserNotificationCount(tenantId);

  console.log(resultWithSso, resultWithoutSso);
}
[inline-code-end]