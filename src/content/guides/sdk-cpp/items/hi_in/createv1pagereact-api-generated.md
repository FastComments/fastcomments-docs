## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| title | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## उदाहरण

[inline-code-attrs-start title = 'createV1PageReact उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto urlId = utility::string_t(U("https://example.com/articles/789"));
boost::optional<utility::string_t> title = utility::string_t(U("Understanding Async C++"));
api->createV1PageReact(tenantId, urlId, title).then([](pplx::task<std::shared_ptr<CreateV1PageReact>> task) {
    try {
        auto response = task.get();
    } catch (const std::exception&) {
    }
});
[inline-code-end]