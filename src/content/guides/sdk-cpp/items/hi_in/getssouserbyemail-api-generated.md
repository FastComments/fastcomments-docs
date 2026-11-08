## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| email | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetSSOUserByEmailAPIResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getSSOUserByEmail उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto correlationId = boost::optional<utility::string_t>(utility::conversions::to_string_t("corr-001"));

api->getSSOUserByEmail(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("user@example.com")
).then([](pplx::task<std::shared_ptr<GetSSOUserByEmailAPIResponse>> t) {
    try {
        auto response = t.get();
    } catch (const std::exception&) {
    }
});
[inline-code-end]