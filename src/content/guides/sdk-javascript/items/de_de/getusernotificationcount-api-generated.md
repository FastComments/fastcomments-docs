## Parameter

| Name     | Typ    | Erforderlich | Beschreibung |
|----------|--------|--------------|--------------|
| tenantId | string | Ja           |              |
| sso      | string | Nein         |              |

## Antwort

Rückgabe: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCountResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getUserNotificationCount Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo(): Promise<void> {
  const tenantId: string = "acme-corp-001";
  const ssoToken: string = "sso-token-xyz789";

  const countWithoutSso: GetUserNotificationCountResponse = await getUserNotificationCount(tenantId);
  const countWithSso: GetUserNotificationCountResponse = await getUserNotificationCount(tenantId, ssoToken);
}

demo();
[inline-code-end]