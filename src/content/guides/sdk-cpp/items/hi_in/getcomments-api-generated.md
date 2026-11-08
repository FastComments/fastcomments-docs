## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetCommentsOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/APIGetCommentsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getComments उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetCommentsOptions options;
options.page = 1;
options.limit = 50;
options.sort = U("newest");
options.authorEmail = boost::optional<utility::string_t>(U("user@example.com"));
api->getComments(tenantId, options).then([](pplx::task<std::shared_ptr<APIGetCommentsResponse>> task) {
    try {
        auto response = task.get();
        // आवश्यकतानुसार response का उपयोग करें
    } catch (const std::exception& e) {
        // त्रुटि को संभालें
    }
});
[inline-code-end]