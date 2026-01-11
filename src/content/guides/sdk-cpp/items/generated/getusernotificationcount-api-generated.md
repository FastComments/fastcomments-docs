## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`GetUserNotificationCount_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserNotificationCount_200_response.h)

## Example

[inline-code-attrs-start title = 'getUserNotificationCount Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->getUserNotificationCount(tenantId, sso).then([](pplx::task<std::shared_ptr<GetUserNotificationCount_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            std::cout << "User notification count retrieved successfully" << std::endl;
        } else {
            std::cout << "No notification count returned" << std::endl;
        }
    } catch (const std::exception& e) {
        std::cerr << "Failed to get notification count: " << e.what() << std::endl;
    }
});
[inline-code-end]
