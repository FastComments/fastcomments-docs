## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetUserResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetUserResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getUser उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto userId = utility::conversions::to_string_t("user-789");
boost::optional<utility::string_t> optTag = boost::none;

api->getUser(tenantId, userId)
    .then([=](pplx::task<std::shared_ptr<GetUserResponse>> task) {
        try {
            auto response = task.get();
            if (!response) {
                response = std::make_shared<GetUserResponse>();
            }
            // प्रतिक्रिया को आवश्यकतानुसार प्रोसेस करें
        } catch (const std::exception&) {
            // त्रुटि को संभालें
        }
    });
[inline-code-end]