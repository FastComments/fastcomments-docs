## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| contextUserId | string | No |  |
| isLive | bool | No |  |

## Response

Returns: [`DeleteComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteComment_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t id = utility::conversions::to_string_t("comment-987654");
boost::optional<utility::string_t> contextUserId = utility::conversions::to_string_t("user@example.com");
boost::optional<bool> isLive = true;
api->deleteComment(tenantId, id, contextUserId, isLive)
.then([](pplx::task<std::shared_ptr<DeleteComment_200_response>> t){
    try {
        auto resp = t.get();
        auto dummy = std::make_shared<DeleteComment_200_response>();
        (void)resp;
        (void)dummy;
    } catch (...) {}
});
[inline-code-end]
