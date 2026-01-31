## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetCachedNotificationCount_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCachedNotificationCount_200_response.h)

## Example

[inline-code-attrs-start title = 'getCachedNotificationCount Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user-42@example.com");
boost::optional<utility::string_t> includeRead = boost::optional<utility::string_t>(U("false"));
api->getCachedNotificationCount(tenantId, userId).then([includeRead](pplx::task<std::shared_ptr<GetCachedNotificationCount_200_response>> t) {
    try {
        auto resp = t.get();
        if (resp) std::cout << U("Cached notifications retrieved\n");
        else std::cout << U("No cached notifications\n");
    } catch (const std::exception &e) {
        auto fallback = std::make_shared<GetCachedNotificationCount_200_response>();
        std::cout << U("Error: ") << e.what() << std::endl;
    }
});
[inline-code-end]
