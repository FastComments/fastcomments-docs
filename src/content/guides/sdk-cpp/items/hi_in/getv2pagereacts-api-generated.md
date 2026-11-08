## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetV2PageReacts.h)

## उदाहरण

[inline-code-attrs-start title = 'getV2PageReacts उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t("my-tenant-123");
auto urlId = utility::string_t("https://example.com/article/42");
boost::optional<utility::string_t> locale = utility::string_t("en-US");
api->getV2PageReacts(tenantId, urlId)
    .then([locale](std::shared_ptr<GetV2PageReacts> reacts){
        if (locale) {}
    })
    .then([](pplx::task<void> t){
        try { t.get(); } catch (...) {}
    });
[inline-code-end]