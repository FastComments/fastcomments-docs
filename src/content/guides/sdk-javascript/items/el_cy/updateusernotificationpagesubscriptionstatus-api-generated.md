Enable ή απενεργοποιήστε τις ειδοποιήσεις για μια σελίδα. Όταν οι χρήστες είναι εγγεγραμμένοι σε μια σελίδα, δημιουργούνται ειδοποιήσεις για νέες κύριες σχόλια, και επίσης

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| url | string | Ναι |  |
| pageTitle | string | Ναι |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | Ναι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationPageSubscriptionStatusResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateUserNotificationPageSubscriptionStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-2024";
  const urlId: string = "page-5678";
  const url: string = "https://example.com/articles/typescript-tips";
  const pageTitle: string = "Top TypeScript Tips";
  const subscribedOrUnsubscribed: UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum =
    UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed;
  const sso: string = "sso-token-xyz";

  const resultWithSso: UpdateUserNotificationPageSubscriptionStatusResponse = await updateUserNotificationPageSubscriptionStatus(
    tenantId,
    urlId,
    url,
    pageTitle,
    subscribedOrUnsubscribed,
    sso
  );

  const resultWithoutSso: UpdateUserNotificationPageSubscriptionStatusResponse = await updateUserNotificationPageSubscriptionStatus(
    tenantId,
    urlId,
    url,
    pageTitle,
    subscribedOrUnsubscribed
  );
})();
[inline-code-end]