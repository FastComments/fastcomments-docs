## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| id | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## उदाहरण

[inline-code-attrs-start title = 'deleteV2PageReact उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto urlId = utility::conversions::to_string_t("page-456");
auto reactId = utility::conversions::to_string_t("react-789");
boost::optional<utility::string_t> correlationId = utility::conversions::to_string_t("corr-001");

api->deleteV2PageReact(tenantId, urlId, reactId)
   .then([](pplx::task<std::shared_ptr<CreateV1PageReact>> t) {
        auto result = t.get();
    });
[inline-code-end]