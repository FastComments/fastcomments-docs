## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateModeratorBody | UpdateModeratorBody | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'updateModerator Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("mod-456");
UpdateModeratorBody updateBody;
updateBody.email = boost::optional<utility::string_t>(U("moderator@example.com"));
updateBody.displayName = utility::string_t(U("Alice Moderator"));
api->updateModerator(tenantId, id, updateBody)
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
