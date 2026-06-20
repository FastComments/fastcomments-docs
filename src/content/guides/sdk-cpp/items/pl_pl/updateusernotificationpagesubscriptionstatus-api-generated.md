---
Włącz lub wyłącz powiadomienia dla strony. Gdy użytkownicy są subskrybowani na stronę, powiadomienia są tworzone dla nowych komentarzy głównych, a także

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| url | string | Tak |  |
| pageTitle | string | Tak |  |
| subscribedOrUnsubscribed | string | Tak |  |
| sso | string | Nie |  |

## Response

Zwraca: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserNotificationPageSubscriptionStatusResponse.h)

## Example

[inline-code-attrs-start title = 'Przykład updateUserNotificationPageSubscriptionStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> sso(utility::conversions::to_string_t("sso-token-abc123"));
api->updateUserNotificationPageSubscriptionStatus(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("article-456"),
    utility::conversions::to_string_t("https://www.example.com/posts/456"),
    utility::conversions::to_string_t("How to Test C++ SDK"),
    utility::conversions::to_string_t("subscribed"),
    sso
).then([](pplx::task<std::shared_ptr<UpdateUserNotificationPageSubscriptionStatusResponse>> t){
    try {
        auto resp = t.get();
        auto copy = std::make_shared<UpdateUserNotificationPageSubscriptionStatusResponse>(*resp);
        (void)copy;
    } catch (const std::exception&) { }
});
[inline-code-end]

---