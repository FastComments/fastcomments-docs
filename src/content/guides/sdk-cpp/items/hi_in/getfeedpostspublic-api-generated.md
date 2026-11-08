req
tenantId
afterId

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetFeedPostsPublicOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PublicFeedPostsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getFeedPostsPublic उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = GetFeedPostsPublicOptions{};
options.limit = boost::optional<int>{20};
options.before = boost::optional<utility::string_t>{U("2023-01-01T00:00:00Z")};

api->getFeedPostsPublic(U("my-tenant-123"), options).then([](pplx::task<std::shared_ptr<PublicFeedPostsResponse>> task){
    try{
        auto response = task.get();
        auto processed = std::make_shared<PublicFeedPostsResponse>(*response);
        // processed का आवश्यकतानुसार उपयोग करें
    }catch(const std::exception&){
        // त्रुटि को संभालें
    }
});
[inline-code-end]