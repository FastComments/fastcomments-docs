## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCountResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo(): Promise<void> {
  const tenantId: string = "acme-corp-001";
  const ssoToken: string = "sso-token-xyz789";

  const countWithoutSso: GetUserNotificationCountResponse = await getUserNotificationCount(tenantId);
  const countWithSso: GetUserNotificationCountResponse = await getUserNotificationCount(tenantId, ssoToken);
}

demo();
[inline-code-end]

---