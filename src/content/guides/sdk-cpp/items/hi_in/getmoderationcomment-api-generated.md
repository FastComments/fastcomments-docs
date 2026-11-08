## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| options | const GetModerationCommentOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/ModerationAPICommentResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getModerationComment उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("comment-456");
GetModerationCommentOptions options;
options.includeDeleted = boost::optional<bool>(true);
options.locale = boost::optional<utility::string_t>(utility::conversions::to_string_t("en-US"));
api->getModerationComment(tenantId, commentId, options)
    .then([](pplx::task<std::shared_ptr<ModerationAPICommentResponse>> task) {
        try {
            auto response = task.get();
            // प्रतिक्रिया को आवश्यकतानुसार प्रोसेस करें
        } catch (const std::exception& ex) {
            // त्रुटि को संभालें
        }
    });
[inline-code-end]