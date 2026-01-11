## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updatableCommentParams | UpdatableCommentParams | Yes |  |
| contextUserId | string | No |  |
| doSpamCheck | bool | No |  |
| isLive | bool | No |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'updateComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456789");
auto paramsPtr = std::make_shared<UpdatableCommentParams>();
paramsPtr->content = U("Clarified the point about API usage and fixed a typo.");
boost::optional<utility::string_t> contextUserId = U("moderator@example.com");
boost::optional<bool> doSpamCheck = true;
boost::optional<bool> isLive = false;
api->updateComment(tenantId, commentId, *paramsPtr, contextUserId, doSpamCheck, isLive)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) std::cout << "Comment updated successfully\n";
        else std::cout << "No response received\n";
    } catch (const std::exception &e) {
        std::cerr << "Update failed: " << e.what() << std::endl;
    }
});
[inline-code-end]
