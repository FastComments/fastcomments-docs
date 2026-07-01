## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| notificationId | string | Ναι |  |
| newStatus | string | Ναι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationStatusResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'updateUserNotificationStatus Παράδειγμα'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->updateUserNotificationStatus(
    U("my-tenant-123"),
    U("notif-456"),
    U("read"),
    boost::optional<utility::string_t>(U("sso-token-abc"))
).then([](std::shared_ptr<UpdateUserNotificationStatusResponse> resp) {
}).wait();
[inline-code-end]

---