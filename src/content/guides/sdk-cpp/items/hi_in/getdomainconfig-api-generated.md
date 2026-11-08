## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| domain | string | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetDomainConfigResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getDomainConfig उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t domain = U("myblog.example.com");

api->getDomainConfig(tenantId, domain)
    .then([](std::shared_ptr<GetDomainConfigResponse> response) {
        if (!response) return;
        boost::optional<bool> moderationEnabled = response->moderationEnabled;
        boost::optional<std::string> theme = response->theme;
        if (moderationEnabled && *moderationEnabled) {
            // सक्षम मॉडरेशन को हैंडल करें
        }
        if (theme) {
            // theme मान का उपयोग करें
        }
    })
    .wait();
[inline-code-end]