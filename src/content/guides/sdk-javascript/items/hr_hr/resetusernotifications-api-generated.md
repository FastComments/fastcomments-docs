## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| afterId | string | Ne |  |
| afterCreatedAt | number | Ne |  |
| unreadOnly | boolean | Ne |  |
| dmOnly | boolean | Ne |  |
| noDm | boolean | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## Primjer

[inline-code-attrs-start title = 'resetUserNotifications Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const afterId: string = "notif-2023-09-01";
  const afterCreatedAt: number = 1693526400; // Unix timestamp
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