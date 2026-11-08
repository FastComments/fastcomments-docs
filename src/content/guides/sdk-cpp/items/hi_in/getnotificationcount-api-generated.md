## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetNotificationCountOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetNotificationCountResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getNotificationCount उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
GetNotificationCountOptions options;
options.filter = boost::optional<utility::string_t>(U("unread"));
options.maxCount = boost::optional<int>(10);
api->getNotificationCount(tenantId, options)
    .then([](pplx::task<std::shared_ptr<GetNotificationCountResponse>> t) {
        try {
            auto response = t.get();
        } catch (const std::exception& ex) {
        }
    });
[inline-code-end]