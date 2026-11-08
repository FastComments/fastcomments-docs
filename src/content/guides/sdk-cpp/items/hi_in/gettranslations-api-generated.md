## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| r_namespace | string | हाँ |  |
| component | string | हाँ |  |
| options | const GetTranslationsOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetTranslationsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getTranslations उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t ns = U("my-tenant-123");
utility::string_t comp = U("comments");
auto optsPtr = std::make_shared<GetTranslationsOptions>();
optsPtr->language = boost::make_optional(U("en"));
optsPtr->fallback = boost::none;
api->getTranslations(ns, comp, *optsPtr)
    .then([](pplx::task<std::shared_ptr<GetTranslationsResponse>> t) {
        try {
            auto resp = t.get();
        } catch (const std::exception& e) {
        }
    });
[inline-code-end]