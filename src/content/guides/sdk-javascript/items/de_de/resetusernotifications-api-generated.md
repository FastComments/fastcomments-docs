---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| afterId | string | Nein |  |
| afterCreatedAt | number | Nein |  |
| unreadOnly | boolean | Nein |  |
| dmOnly | boolean | Nein |  |
| noDm | boolean | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'resetUserNotifications Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const afterId: string = "notif-2023-09-01";
  const afterCreatedAt: number = 1693526400; // Unix-Zeitstempel
  const unreadOnly: boolean = true;
  const dmOnly: boolean = false;

  const response: ResetUserNotificationsResponse = await resetUserNotifications(
    tenantId,
    afterId,
    afterCreatedAt,
    unreadOnly,
    dmOnly
  );

  console.log(response);
})();
[inline-code-end]

---