## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Одговор

Враћа: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/APIModerateGetUserBanPreferencesResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getUserBanPreference'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->getUserBanPreference(tenantId, sso)
    .then([](pplx::task<std::shared_ptr<APIModerateGetUserBanPreferencesResponse>> t) {
        try {
            auto resp = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]