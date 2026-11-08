## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetUserNotificationsOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetMyNotificationsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getUserNotifications उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::string_t("my-tenant-123");
GetUserNotificationsOptions options;
options.limit = boost::optional<int>(20);
options.unreadOnly = boost::optional<bool>(true);
api->getUserNotifications(tenantId, options)
    .then([](pplx::task<std::shared_ptr<GetMyNotificationsResponse>> task){
        auto resp = task.get();
        auto notifications = std::make_shared<GetMyNotificationsResponse>(*resp);
    });
[inline-code-end]