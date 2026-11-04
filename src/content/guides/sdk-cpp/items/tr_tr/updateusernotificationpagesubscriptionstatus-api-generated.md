Bir sayfa için bildirimleri etkinleştirin veya devre dışı bırakın. Kullanıcılar bir sayfaya abone olduğunda, yeni kök yorumlar için ve ayrıca

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| url | string | Evet |  |
| pageTitle | string | Evet |  |
| subscribedOrUnsubscribed | string | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/UpdateUserNotificationPageSubscriptionStatusResponse.h)

## Örnek

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
utility::string_t url = U("https://example.com/articles/awesome-article");
utility::string_t pageTitle = U("Awesome Article");
utility::string_t subscription = U("subscribed");
boost::optional<utility::string_t> sso = boost::make_optional<utility::string_t>(U("sso-token-789"));

api->updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscription, sso)
    .then([](std::shared_ptr<UpdateUserNotificationPageSubscriptionStatusResponse> resp) {
        // gerekiyorsa resp'yi işleyin
    });
[inline-code-end]