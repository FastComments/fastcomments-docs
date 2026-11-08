req
tenantId
afterId

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetFeedPostsOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetFeedPostsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getFeedPosts उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto opts = std::make_shared<GetFeedPostsOptions>();
opts->maxResults = boost::optional<int>(50);
opts->cursor = boost::optional<utility::string_t>(U("next-cursor"));
api->getFeedPosts(U("my-tenant-123"), *opts).then([](std::shared_ptr<GetFeedPostsResponse> resp) {
    auto count = resp->posts.size();
});
[inline-code-end]