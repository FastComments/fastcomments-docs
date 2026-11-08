## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetTenantsOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetTenantsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getTenants उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GetTenantsOptions options;
options.includeDeleted = boost::make_optional(false);
options.searchTerm = boost::make_optional(utility::string_t(U("enterprise")));

api->getTenants(utility::string_t(U("my-tenant-123")), options)
    .then([](std::shared_ptr<GetTenantsResponse> response) {
    })
    .then([](pplx::task<void> t){ try{ t.get(); }catch(...){} });
[inline-code-end]