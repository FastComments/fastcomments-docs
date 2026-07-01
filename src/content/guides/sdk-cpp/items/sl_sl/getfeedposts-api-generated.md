req
tenantId
afterId

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| options | const GetFeedPostsOptions& | Da |  |

## Odgovor

Vrne: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetFeedPostsResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer getFeedPosts'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto opts = std::make_shared<GetFeedPostsOptions>();
opts->maxResults = boost::optional<int>(50);
opts->cursor = boost::optional<utility::string_t>(U("next-cursor"));
api->getFeedPosts(U("my-tenant-123"), *opts).then([](std::shared_ptr<GetFeedPostsResponse> resp) {
    auto count = resp->posts.size();
});
[inline-code-end]