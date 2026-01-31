## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| urlId | string | No |  |
| fromCommentId | string | No |  |
| viewed | bool | No |  |
| type | string | No |  |
| skip | double | No |  |

## Response

Returns: [`GetNotifications_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetNotifications_200_response.h)

## Example

[inline-code-attrs-start title = 'getNotifications Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> userId = U("user@example.com");
boost::optional<utility::string_t> urlId = U("https://blog.example.com/posts/42");
boost::optional<utility::string_t> fromCommentId = U("cmt-98765");
boost::optional<bool> viewed = false;
boost::optional<utility::string_t> type = U("mention");
boost::optional<double> skip = 0.0;

api->getNotifications(tenantId, userId, urlId, fromCommentId, viewed, type, skip)
.then([](pplx::task<std::shared_ptr<GetNotifications_200_response>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<GetNotifications_200_response>();
    } catch (const std::exception&) {}
});
[inline-code-end]
