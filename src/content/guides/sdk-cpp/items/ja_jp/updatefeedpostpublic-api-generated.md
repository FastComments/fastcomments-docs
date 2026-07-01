## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| updateFeedPostParams | UpdateFeedPostParams | Yes |  |
| options | const UpdateFeedPostPublicOptions& | Yes |  |

## レスポンス

戻り値: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostResponse.h)

## 例

[inline-code-attrs-start title = 'updateFeedPostPublic の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("post-456");
UpdateFeedPostParams params;
params.title = boost::optional<utility::string_t>(U("Updated Title"));
params.body = boost::optional<utility::string_t>(U("Updated content of the post."));
UpdateFeedPostPublicOptions options;
options.notifyFollowers = boost::optional<bool>(true);
api->updateFeedPostPublic(tenantId, postId, params, options)
    .then([](std::shared_ptr<CreateFeedPostResponse> resp) {
        auto result = resp;
    });
[inline-code-end]