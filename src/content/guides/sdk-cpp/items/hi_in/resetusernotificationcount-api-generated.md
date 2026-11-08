## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/ResetUserNotificationsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'resetUserNotificationCount उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto resetTask = api->resetUserNotificationCount(
    U("my-tenant-123"),
    boost::optional<utility::string_t>(U("user@example.com"))
).then([](std::shared_ptr<ResetUserNotificationsResponse> resp){
});
[inline-code-end]