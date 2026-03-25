## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetUserNotificationCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCount200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = '9f1e2d3c-4b5a-6d7e-8f90-123456789abc';
  const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI0MjMifQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
  const resultWithSSO: GetUserNotificationCount200Response = await getUserNotificationCount(tenantId, ssoToken);
  const resultWithoutSSO: GetUserNotificationCount200Response = await getUserNotificationCount(tenantId);
  console.log(resultWithSSO, resultWithoutSSO);
})();
[inline-code-end]

---