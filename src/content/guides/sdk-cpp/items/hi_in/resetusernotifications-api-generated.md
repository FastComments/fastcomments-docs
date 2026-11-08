## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const ResetUserNotificationsOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/ResetUserNotificationsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'resetUserNotifications उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
ResetUserNotificationsOptions options;
options.email = boost::optional<utility::string_t>(U("user@example.com"));
api->resetUserNotifications(tenantId, options)
    .then([](std::shared_ptr<ResetUserNotificationsResponse> resp) {
        // प्रतिक्रिया प्रोसेस करें
    });
[inline-code-end]