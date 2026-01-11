## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| pageSize | int32_t | No |  |
| afterId | string | No |  |
| includeContext | bool | No |  |
| afterCreatedAt | int64_t | No |  |
| unreadOnly | bool | No |  |
| dmOnly | bool | No |  |
| noDm | bool | No |  |
| includeTranslations | bool | No |  |
| sso | string | No |  |

## Response

Returns: [`GetUserNotifications_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserNotifications_200_response.h)

## Example

[inline-code-attrs-start title = 'getUserNotifications Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId(U("my-tenant-123"));
boost::optional<int32_t> pageSize = 25;
boost::optional<utility::string_t> afterId = utility::string_t(U("notif_456"));
boost::optional<bool> includeContext = true;
boost::optional<int64_t> afterCreatedAt = 1625097600000LL;
boost::optional<bool> unreadOnly = true;
boost::optional<bool> dmOnly = false;
boost::optional<bool> noDm = false;
boost::optional<bool> includeTranslations = true;
boost::optional<utility::string_t> sso = utility::string_t(U("user@example.com"));

api->getUserNotifications(tenantId, pageSize, afterId, includeContext, afterCreatedAt, unreadOnly, dmOnly, noDm, includeTranslations, sso)
.then([](pplx::task<std::shared_ptr<GetUserNotifications_200_response>> t){
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<GetUserNotifications_200_response>();
        std::cout << "Notifications fetched\n";
    } catch (const std::exception &e) {
        std::cerr << "Failed: " << e.what() << '\n';
    }
});
[inline-code-end]
