## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetTenantManualBadgesResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getManualBadges उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> sso = boost::make_optional(U("user@example.com"));

api->getManualBadges(tenantId, sso)
    .then([](pplx::task<std::shared_ptr<GetTenantManualBadgesResponse>> t) {
        try {
            auto response = t.get();
            // प्रतिक्रिया प्रोसेस करें, जैसे, response->badgeList
        } catch (const std::exception& ex) {
            // त्रुटि को संभालें
        }
    });
[inline-code-end]