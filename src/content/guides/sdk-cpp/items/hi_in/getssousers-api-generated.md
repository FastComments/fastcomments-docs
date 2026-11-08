## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| skip | int32_t | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetSSOUsersResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getSSOUsers उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<int32_t> skip = 25;
api->getSSOUsers(tenantId, skip).then([](pplx::task<std::shared_ptr<GetSSOUsersResponse>> t) {
    try {
        auto response = t.get();
    } catch (const std::exception&) {
    }
});
[inline-code-end]