## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| afterId | string | No |  |
| afterCreatedAt | number | No |  |
| unreadOnly | boolean | No |  |
| dmOnly | boolean | No |  |
| noDm | boolean | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio resetUserNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const afterId: string = "notif-2023-09-01";
  const afterCreatedAt: number = 1693526400; // Timestamp Unix
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