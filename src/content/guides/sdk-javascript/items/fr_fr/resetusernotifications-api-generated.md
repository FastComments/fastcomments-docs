## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| afterId | string | Non |  |
| afterCreatedAt | number | Non |  |
| unreadOnly | boolean | Non |  |
| dmOnly | boolean | Non |  |
| noDm | boolean | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'resetUserNotifications Exemple'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const afterId: string = "notif-2023-09-01";
  const afterCreatedAt: number = 1693526400; // horodatage Unix
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