Ενεργοποίηση ή απενεργοποίηση ειδοποιήσεων για μια σελίδα. Όταν οι χρήστες είναι εγγεγραμμένοι σε μια σελίδα, δημιουργούνται ειδοποιήσεις
για νέα σχόλια ρίζας, καθώς και

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| url | string | Ναι |  |
| pageTitle | string | Ναι |  |
| subscribedOrUnsubscribed | string | Ναι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/UpdateUserNotificationPageSubscriptionStatusResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα updateUserNotificationPageSubscriptionStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
utility::string_t url = U("https://example.com/articles/awesome-article");
utility::string_t pageTitle = U("Awesome Article");
utility::string_t subscription = U("subscribed");
boost::optional<utility::string_t> sso = boost::make_optional<utility::string_t>(U("sso-token-789"));

api->updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscription, sso)
    .then([](std::shared_ptr<UpdateUserNotificationPageSubscriptionStatusResponse> resp) {
        // επεξεργασία resp αν χρειάζεται
    });
[inline-code-end]