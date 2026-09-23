## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| afterId | string | Όχι |  |
| afterCreatedAt | number | Όχι |  |
| unreadOnly | boolean | Όχι |  |
| dmOnly | boolean | Όχι |  |
| noDm | boolean | Όχι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'resetUserNotifications Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const afterId: string = "notif-2023-09-01";
  const afterCreatedAt: number = 1693526400; // Χρονική σήμανση Unix
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