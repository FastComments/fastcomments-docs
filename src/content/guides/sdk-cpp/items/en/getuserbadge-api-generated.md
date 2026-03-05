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
utility::string_t id = U("user@example.com");
boost::optional<utility::string_t> locale = boost::optional<utility::string_t>(U("en-US"));
api->getUserBadge(tenantId, id).then([=](pplx::task<std::shared_ptr<GetUserBadge_200_response>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<GetUserBadge_200_response>();
        return resp;
    } catch (const std::exception&) {
        return std::make_shared<GetUserBadge_200_response>();
    }
}).then([](std::shared_ptr<GetUserBadge_200_response> result) {
    (void)result;
});
[inline-code-end]
