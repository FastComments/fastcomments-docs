## Parameters

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| sso | string | Όχι |  |

## Response

Επιστρέφει: [`ResetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationCountResponse.ts)

## Example

[inline-code-attrs-start title = 'resetUserNotificationCount Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoResetCount() {
  const tenantId: string = "acme-corp-tenant";
  const sso: string = "sso-user-9876";

  // Κλήση με προαιρετική παράμετρο sso
  const resultWithSso: ResetUserNotificationCountResponse = await resetUserNotificationCount(tenantId, sso);

  // Κλήση χωρίς προαιρετική παράμετρο sso
  const resultWithoutSso: ResetUserNotificationCountResponse = await resetUserNotificationCount(tenantId);

  console.log(resultWithSso, resultWithoutSso);
}
[inline-code-end]