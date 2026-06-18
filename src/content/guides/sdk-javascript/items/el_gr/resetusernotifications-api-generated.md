## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| afterId | string | Όχι |  |
| afterCreatedAt | number | Όχι |  |
| unreadOnly | boolean | Όχι |  |
| dmOnly | boolean | Όχι |  |
| noDm | boolean | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotifications200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα resetUserNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9b1f2";
const afterId: string = "notification_0001";
const afterCreatedAt: number = Date.now() - 60 * 60 * 1000; // πριν από μία ώρα
const unreadOnly: boolean = true;
const dmOnly: boolean = false;
const noDm: boolean = false;
const sso: string = "sso_session_7f2d";
const result: ResetUserNotifications200Response = await resetUserNotifications(
  tenantId,
  afterId,
  afterCreatedAt,
  unreadOnly,
  dmOnly,
  noDm,
  sso
);
[inline-code-end]

---