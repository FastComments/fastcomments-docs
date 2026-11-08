## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetTenantUserResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getTenantUser उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto userId = utility::conversions::to_string_t("user@example.com");
api->getTenantUser(tenantId, userId)
    .then([](pplx::task<std::shared_ptr<GetTenantUserResponse>> task) {
        try {
            auto response = task.get();
            // आवश्यकतानुसार response का उपयोग करें
        } catch (const std::exception&) {
            // त्रुटि हैंडलिंग
        }
    });
[inline-code-end]