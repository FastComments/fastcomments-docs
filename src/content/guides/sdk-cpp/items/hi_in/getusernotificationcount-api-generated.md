## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetUserNotificationCountResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getUserNotificationCount उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("user@example.com");
api->getUserNotificationCount(tenantId, sso).then([](pplx::task<std::shared_ptr<GetUserNotificationCountResponse>> t){
    try{
        auto resp = t.get();
        // resp का आवश्यकतानुसार उपयोग करें
    }catch(const std::exception&){
        // त्रुटि को संभालें
    }
});
[inline-code-end]