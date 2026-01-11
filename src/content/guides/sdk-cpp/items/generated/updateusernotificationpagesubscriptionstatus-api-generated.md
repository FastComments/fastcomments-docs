## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| url | string | Yes |  |
| pageTitle | string | Yes |  |
| subscribedOrUnsubscribed | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`UpdateUserNotificationStatus_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationStatus_200_response.h)

## Example

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
utility::string_t url = U("https://www.example.com/articles/fastcomments-integration");
utility::string_t pageTitle = U("FastComments Integration Guide");
utility::string_t subscribedOrUnsubscribed = U("subscribed");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-token-74a9"));
api->updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed, sso)
    .then([](pplx::task<std::shared_ptr<UpdateUserNotificationStatus_200_response>> task){
        try {
            auto resp = task.get();
            if (!resp) {
                auto fallback = std::make_shared<UpdateUserNotificationStatus_200_response>();
                (void)fallback;
            } else {
                (void)resp;
            }
        } catch (const std::exception& e) {
            (void)e;
        }
    });
[inline-code-end]
