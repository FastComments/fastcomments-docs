## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |

## Response

Returns: [`GetUserBadgeProgressById_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserBadgeProgressById_200_response.h)

## Example

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> userIdOpt = utility::string_t(U("user@example.com"));
auto task = api->getUserBadgeProgressByUserId(tenantId, userIdOpt.value())
    .then([](pplx::task<std::shared_ptr<GetUserBadgeProgressById_200_response>> prev) {
        try {
            auto resp = prev.get();
            if (!resp) return std::shared_ptr<GetUserBadgeProgressById_200_response>{};
            auto copy = std::make_shared<GetUserBadgeProgressById_200_response>(*resp);
            return copy;
        } catch (...) {
            return std::shared_ptr<GetUserBadgeProgressById_200_response>{};
        }
    });
[inline-code-end]
