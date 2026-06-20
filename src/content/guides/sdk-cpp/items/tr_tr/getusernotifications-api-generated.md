## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Hayır |  |
| pageSize | int32_t | Hayır |  |
| afterId | string | Hayır |  |
| includeContext | bool | Hayır |  |
| afterCreatedAt | int64_t | Hayır |  |
| unreadOnly | bool | Hayır |  |
| dmOnly | bool | Hayır |  |
| noDm | bool | Hayır |  |
| includeTranslations | bool | Hayır |  |
| includeTenantNotifications | bool | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetMyNotificationsResponse.h)

## Örnek

[inline-code-attrs-start title = 'getUserNotifications Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
api->getUserNotifications(
    tenantId,
    boost::optional<utility::string_t>(U("post-456")),
    boost::optional<int32_t>(50),
    boost::optional<utility::string_t>(U("notif-789")),
    boost::optional<bool>(true),
    boost::optional<int64_t>(1625097600000LL),
    boost::optional<bool>(true),
    boost::optional<bool>(false),
    boost::optional<bool>(false),
    boost::optional<bool>(true),
    boost::optional<bool>(false),
    boost::optional<utility::string_t>(U("user@example.com"))
).then([](pplx::task<std::shared_ptr<GetMyNotificationsResponse>> t){
    try {
        auto resp = t.get();
        if(!resp) resp = std::make_shared<GetMyNotificationsResponse>();
        // resp kullanın, örn. alanları inceleyin
    } catch(const std::exception &e) {
    }
});
[inline-code-end]