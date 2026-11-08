## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| skip | double | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetTenantPackagesResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getTenantPackages उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<double> skip = 20.0;
api->getTenantPackages(tenantId, skip)
    .then([](std::shared_ptr<GetTenantPackagesResponse> resp) {
        (void)resp;
    });
[inline-code-end]