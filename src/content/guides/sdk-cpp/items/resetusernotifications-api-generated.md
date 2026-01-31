## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| afterId | string | No |  |
| afterCreatedAt | int64_t | No |  |
| unreadOnly | bool | No |  |
| dmOnly | bool | No |  |
| noDm | bool | No |  |
| sso | string | No |  |

## Response

Returns: [`ResetUserNotifications_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ResetUserNotifications_200_response.h)

## Example

[inline-code-attrs-start title = 'resetUserNotifications Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> afterId = boost::optional<utility::string_t>(U("notif-456"));
boost::optional<int64_t> afterCreatedAt = boost::optional<int64_t>(1672531200);
boost::optional<bool> unreadOnly = boost::optional<bool>(true);
boost::optional<bool> dmOnly = boost::optional<bool>(false);
boost::optional<bool> noDm = boost::optional<bool>(false);
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->resetUserNotifications(tenantId, afterId, afterCreatedAt, unreadOnly, dmOnly, noDm, sso)
.then([](pplx::task<std::shared_ptr<ResetUserNotifications_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto copy = std::make_shared<ResetUserNotifications_200_response>(*resp);
            std::cout << "Notifications reset\n";
        } else {
            std::cerr << "No response received\n";
        }
    } catch (const std::exception& e) {
        std::cerr << "Reset failed: " << e.what() << '\n';
    }
});
[inline-code-end]
