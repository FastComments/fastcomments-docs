## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|------------|--------------|
| tenantId | string | Ja |  |
| afterId | string | Nee |  |
| afterCreatedAt | number | Nee |  |
| unreadOnly | boolean | Nee |  |
| dmOnly | boolean | Nee |  |
| noDm | boolean | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`ResetUserNotificationsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse1.ts)

## Voorbeeld

[inline-code-attrs-start title = 'resetUserNotifications Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-001";
  const afterId: string = "notif-123";
  const afterCreatedAt: number = 1697049600; // voorbeeld UNIX-tijdstempel
  const unreadOnly: boolean = true;
  const dmOnly: boolean = false;
  const noDm: boolean = false;
  const sso: string = "sso-token-xyz";

  const result: ResetUserNotificationsResponse1 = await resetUserNotifications(
    tenantId,
    afterId,
    afterCreatedAt,
    unreadOnly,
    dmOnly,
    noDm,
    sso
  );

  console.log(result);
})();
[inline-code-end]

---