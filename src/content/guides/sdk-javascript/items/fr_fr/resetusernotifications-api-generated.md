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

Renvoie : [`ResetUserNotificationsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple resetUserNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-001";
  const afterId: string = "notif-123";
  const afterCreatedAt: number = 1697049600; // exemple d'horodatage UNIX
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