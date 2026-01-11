## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetUserBadge_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserBadge_200_response.h)

## Example

[inline-code-attrs-start title = 'getUserBadge Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("alice@example.com");
boost::optional<utility::string_t> locale = boost::none;
auto defaultResp = std::make_shared<GetUserBadge_200_response>();
api->getUserBadge(tenantId, userId)
    .then([defaultResp](std::shared_ptr<GetUserBadge_200_response> resp) {
        auto result = resp ? resp : defaultResp;
        return result;
    })
    .then([](std::shared_ptr<GetUserBadge_200_response> finalResp) {
        if (finalResp) {
            std::cout << "User badge retrieved successfully\n";
        } else {
            std::cout << "Failed to retrieve user badge\n";
        }
    });
[inline-code-end]
