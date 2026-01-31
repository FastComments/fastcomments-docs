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
auto updatableParams = std::make_shared<UpdatableCommentParams>();
updatableParams->content = U("Updated the comment to clarify reproduction steps and expected result");
boost::optional<utility::string_t> contextUserId = boost::optional<utility::string_t>(U("developer@example.com"));
boost::optional<bool> doSpamCheck = boost::optional<bool>(true);
boost::optional<bool> isLive = boost::optional<bool>(true);

api->updateComment(tenantId, commentId, *updatableParams, contextUserId, doSpamCheck, isLive)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> previous){
    try {
        auto response = previous.get();
        if (response) {}
    } catch(const std::exception&) {}
});
[inline-code-end]
