## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetVotesResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getVotes उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto urlId = utility::string_t(U("article-456"));
boost::optional<utility::string_t> extraHeader = boost::none;

api->getVotes(tenantId, urlId).then([=](pplx::task<std::shared_ptr<GetVotesResponse>> task) {
    try {
        auto original = task.get();
        auto response = std::make_shared<GetVotesResponse>(*original);
    } catch (...) {
        // त्रुटि प्रबंधन
    }
});
[inline-code-end]