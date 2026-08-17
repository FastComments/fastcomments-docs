## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Одговор

Враћа: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetUserNotificationCountResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getUserNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("user@example.com");
api->getUserNotificationCount(tenantId, sso).then([](pplx::task<std::shared_ptr<GetUserNotificationCountResponse>> t){
    try{
        auto resp = t.get();
        // користи одговор по потреби
    }catch(const std::exception&){
        // обради грешку
    }
});
[inline-code-end]