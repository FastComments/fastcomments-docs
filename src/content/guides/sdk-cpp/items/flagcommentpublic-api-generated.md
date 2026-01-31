## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| isFlagged | bool | Yes |  |
| sso | string | No |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'flagCommentPublic Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-987654321");
bool isFlagged = true;
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));

api->flagCommentPublic(tenantId, commentId, isFlagged, sso)
    .then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> task) {
        try {
            auto resp = task.get();
            auto result = std::make_shared<FlagCommentPublic_200_response>();
            if (resp) result = resp;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]
