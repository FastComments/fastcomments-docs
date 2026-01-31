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
boost::optional<utility::string_t> sso = utility::string_t(U("user@example.com"));
auto task = api->getUserNotificationCount(tenantId, sso)
    .then([](pplx::task<std::shared_ptr<GetUserNotificationCount_200_response>> prev){
        try {
            auto resp = prev.get();
            auto respCopy = std::make_shared<GetUserNotificationCount_200_response>(*resp);
        } catch (const std::exception&) {
        }
    });
[inline-code-end]
