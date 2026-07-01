リクエスト
tenantId
afterId

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetFeedPostsOptions& | Yes |  |

## レスポンス

返却: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetFeedPostsResponse.h)

## 例

[inline-code-attrs-start title = 'getFeedPosts の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto opts = std::make_shared<GetFeedPostsOptions>();
opts->maxResults = boost::optional<int>(50);
opts->cursor = boost::optional<utility::string_t>(U("next-cursor"));
api->getFeedPosts(U("my-tenant-123"), *opts).then([](std::shared_ptr<GetFeedPostsResponse> resp) {
    auto count = resp->posts.size();
});
[inline-code-end]