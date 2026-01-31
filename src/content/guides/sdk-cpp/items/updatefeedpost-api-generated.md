## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| feedPost | FeedPost | Yes |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'updateFeedPost Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("feedpost-789");
FeedPost feedPost;
feedPost.content = utility::string_t(U("Updated post content with clarified wording."));
feedPost.authorEmail = utility::string_t(U("alice@example.com"));
boost::optional<utility::string_t> maybeSummary(U("Edited for clarity"));
feedPost.summary = maybeSummary;
api->updateFeedPost(tenantId, postId, feedPost)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t){
    try {
        auto resp = t.get();
        auto result = std::make_shared<FlagCommentPublic_200_response>(*resp);
        return result;
    } catch(...) {
        return std::shared_ptr<FlagCommentPublic_200_response>();
    }
});
[inline-code-end]
