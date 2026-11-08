## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | const GetUserBadgeProgressListOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/APIGetUserBadgeProgressListResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getUserBadgeProgressList उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetUserBadgeProgressListOptions options;
options.userId = U("user@example.com");
options.page = boost::optional<int>(1);
options.pageSize = boost::optional<int>(20);
api->getUserBadgeProgressList(tenantId, options)
    .then([](std::shared_ptr<APIGetUserBadgeProgressListResponse> resp) {
        if (!resp) return;
        for (const auto& badge : resp->badges) {
            // बैज प्रोसेस करें
        }
    });
[inline-code-end]