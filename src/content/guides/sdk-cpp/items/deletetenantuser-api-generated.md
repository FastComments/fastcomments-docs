## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| deleteComments | string | No |  |
| commentDeleteMode | string | No |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'deleteTenantUser Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user@example.com");
boost::optional<utility::string_t> deleteComments = boost::optional<utility::string_t>(U("true"));
boost::optional<utility::string_t> commentDeleteMode = boost::optional<utility::string_t>(U("soft"));
api->deleteTenantUser(tenantId, id, deleteComments, commentDeleteMode)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto copy = std::make_shared<FlagCommentPublic_200_response>(*resp);
        }
    } catch (const std::exception &e) {
    }
});
[inline-code-end]
