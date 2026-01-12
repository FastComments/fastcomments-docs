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
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> optLocale = boost::optional<utility::string_t>(U("en-US"));
api->getUserBadgeProgressById(tenantId, userId)
    .then([=](std::shared_ptr<GetUserBadgeProgressById_200_response> resp) {
        if(!resp) resp = std::make_shared<GetUserBadgeProgressById_200_response>();
        std::cout << "Badge progress retrieved: " << (resp ? "yes" : "no") << std::endl;
        return resp;
    });
[inline-code-end]
