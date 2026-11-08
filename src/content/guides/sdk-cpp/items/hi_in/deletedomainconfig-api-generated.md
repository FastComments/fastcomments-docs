## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| domain | string | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`DeleteDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/DeleteDomainConfigResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'deleteDomainConfig उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> optTenant = U("my-tenant-123");
boost::optional<utility::string_t> optDomain = U("example.com");

api->deleteDomainConfig(optTenant.value(), optDomain.value())
    .then([](pplx::task<std::shared_ptr<DeleteDomainConfigResponse>> task) {
        try {
            auto response = task.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]