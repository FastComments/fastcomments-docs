## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| addDomainConfigParams | AddDomainConfigParams | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/AddDomainConfigResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'addDomainConfig उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
AddDomainConfigParams params;
params.domain = U("example.com");
params.adminEmail = U("admin@example.com");
params.notes = boost::optional<utility::string_t>(U("Primary domain"));
api->addDomainConfig(tenantId, params).then([](pplx::task<std::shared_ptr<AddDomainConfigResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]