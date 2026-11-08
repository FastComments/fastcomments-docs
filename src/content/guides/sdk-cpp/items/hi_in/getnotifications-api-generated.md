## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetNotificationsOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetNotificationsResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getNotifications उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GetNotificationsOptions options;
options.limit = 20;
options.after = U("cursor-123");
api->getNotifications(U("my-tenant-123"), options)
    .then([](std::shared_ptr<GetNotificationsResponse> resp) {
        (void)resp;
    });
[inline-code-end]