## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetUserBadgeProgressById_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserBadgeProgressById_200_response.h)

## Example

[inline-code-attrs-start title = 'getUserBadgeProgressById Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> maybeTenant = utility::string_t(U("my-tenant-123"));
utility::string_t tenantId = maybeTenant ? maybeTenant.value() : utility::string_t(U("my-tenant-123"));
utility::string_t id = utility::string_t(U("user@example.com"));

api->getUserBadgeProgressById(tenantId, id)
.then([](pplx::task<std::shared_ptr<GetUserBadgeProgressById_200_response>> task) {
    try {
        auto resp = task.get();
        if (resp) {
            auto copy = std::make_shared<GetUserBadgeProgressById_200_response>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]
