## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`FlagComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagComment_200_response.h)

## Example

[inline-code-attrs-start title = 'flagComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto commentId = utility::string_t(U("comment-456"));
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<utility::string_t> anonUserId;
api->flagComment(tenantId, commentId, userId, anonUserId)
.then([](pplx::task<std::shared_ptr<FlagComment_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto copied = std::make_shared<FlagComment_200_response>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]
