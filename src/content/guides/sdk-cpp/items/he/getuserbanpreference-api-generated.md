## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## תשובה

מחזיר: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIModerateGetUserBanPreferencesResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getUserBanPreference'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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