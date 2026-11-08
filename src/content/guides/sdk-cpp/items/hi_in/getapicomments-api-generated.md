## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetApiCommentsOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/ModerationAPIGetCommentsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getApiComments उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = GetApiCommentsOptions{};
options.page = boost::make_optional(2);
options.authorEmail = boost::make_optional<utility::string_t>(U("user@example.com"));
options.includeDeleted = boost::make_optional(false);

api->getApiComments(U("my-tenant-123"), options).then([](pplx::task<std::shared_ptr<ModerationAPIGetCommentsResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]