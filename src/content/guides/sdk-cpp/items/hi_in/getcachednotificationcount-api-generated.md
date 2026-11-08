## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetCachedNotificationCountResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getCachedNotificationCount उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = boost::optional<utility::string_t>(utility::conversions::to_string_t("my-tenant-123"));
auto userId = boost::optional<utility::string_t>(utility::conversions::to_string_t("user-456"));

api->getCachedNotificationCount(tenantId.value(), userId.value())
    .then([](pplx::task<std::shared_ptr<GetCachedNotificationCountResponse>> task) {
        try {
            auto response = task.get();
            // प्रतिक्रिया प्रोसेस करें
        } catch (const std::exception&) {
            // त्रुटि संभालें
        }
    });
[inline-code-end]