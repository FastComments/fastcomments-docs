## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| userId | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/APIGetUserBadgeProgressResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<std::shared_ptr<APIGetUserBadgeProgressResponse>> responseOpt;
api->getUserBadgeProgressByUserId(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("user@example.com"))
    .then([&responseOpt](pplx::task<std::shared_ptr<APIGetUserBadgeProgressResponse>> t) {
        try {
            responseOpt = t.get();
        } catch (...) {
            responseOpt = boost::none;
        }
    });
[inline-code-end]