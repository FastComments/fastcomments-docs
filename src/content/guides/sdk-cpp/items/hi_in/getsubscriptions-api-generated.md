## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| userId | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetSubscriptionsAPIResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getSubscriptions उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenant = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> user = utility::conversions::to_string_t("user@example.com");

api->getSubscriptions(tenant, user).then(
    [](pplx::task<std::shared_ptr<GetSubscriptionsAPIResponse>> t) {
        try {
            auto response = t.get();
            // प्रतिक्रिया प्रोसेस करें
        } catch (const std::exception& e) {
            // त्रुटि संभालें
        }
    }
);
[inline-code-end]