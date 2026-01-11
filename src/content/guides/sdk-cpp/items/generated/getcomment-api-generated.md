## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetComment_200_response.h)

## Example

[inline-code-attrs-start title = 'getComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456");
boost::optional<utility::string_t> requestedBy = boost::optional<utility::string_t>(U("moderator@example.com"));
api->getComment(tenantId, commentId)
.then([](pplx::task<std::shared_ptr<GetComment_200_response>> task) {
    try {
        auto resp = task.get();
        if (resp) {
            auto copy = std::make_shared<GetComment_200_response>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]
