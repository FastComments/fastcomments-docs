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
utility::string_t postId = U("post-456");
FeedPost feedPost;
feedPost.title = U("Scheduled maintenance");
feedPost.content = U("We'll be updating servers at 00:00 UTC.");
feedPost.authorEmail = U("admin@my-domain.com");
feedPost.authorDisplayName = boost::optional<utility::string_t>(U("Site Admin"));
feedPost.pinned = boost::optional<bool>(false);
api->updateFeedPost(tenantId, postId, feedPost)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t){
    try {
        auto resp = t.get();
        auto safeResp = resp ? resp : std::make_shared<FlagCommentPublic_200_response>();
        if (safeResp) std::cout << "Feed post updated successfully\n";
        else std::cout << "Update returned empty response\n";
    } catch (const std::exception &e) {
        std::cerr << "Update failed: " << e.what() << '\n';
    }
});
[inline-code-end]
