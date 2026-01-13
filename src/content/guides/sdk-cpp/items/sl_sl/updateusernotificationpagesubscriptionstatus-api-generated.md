---
Omogočite ali onemogočite obvestila za stran. Ko so uporabniki naročeni na stran, se ustvarijo obvestila za nove osnovne komentarje, in tudi

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| url | string | Da |  |
| pageTitle | string | Da |  |
| subscribedOrUnsubscribed | string | Da |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`UpdateUserNotificationStatus_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationStatus_200_response.h)

## Primer

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
utility::string_t url = U("https://www.example.com/articles/2026/new-feature");
utility::string_t pageTitle = U("New Feature Announcement");
utility::string_t subscribedOrUnsubscribed = U("subscribed");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
auto placeholder = std::make_shared<UpdateUserNotificationStatus_200_response>();
api->updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed, sso)
.then([](pplx::task<std::shared_ptr<UpdateUserNotificationStatus_200_response>> task){
    try {
        auto resp = task.get();
        (void)resp;
    } catch (const std::exception &e) {
        (void)e;
    }
});
[inline-code-end]

---