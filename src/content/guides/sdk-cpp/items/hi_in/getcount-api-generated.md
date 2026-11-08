## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetCountOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/ModerationAPICountCommentsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getCount उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetCountOptions options;
options.userEmail = boost::optional<utility::string_t>(U("user@example.com"));
options.maxResults = boost::optional<int>(100);
api->getCount(tenantId, options).then([](pplx::task<std::shared_ptr<ModerationAPICountCommentsResponse>> task){
    try{
        auto resp = task.get();
        auto copy = std::make_shared<ModerationAPICountCommentsResponse>(*resp);
        std::cout << "Count: " << copy->count << std::endl;
    }catch(const std::exception& e){
        std::cerr << "Error: " << e.what() << std::endl;
    }
});
[inline-code-end]