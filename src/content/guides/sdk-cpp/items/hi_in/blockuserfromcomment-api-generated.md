## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| blockFromCommentParams | BlockFromCommentParams | हाँ |  |
| options | const BlockUserFromCommentOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`BlockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/BlockSuccess.h)

## उदाहरण

[inline-code-attrs-start title = 'blockUserFromComment उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("comment-789");
BlockFromCommentParams params;
params.reason = utility::conversions::to_string_t("Inappropriate content");
params.durationDays = boost::optional<int>(30);
BlockUserFromCommentOptions options;
options.notifyUser = boost::optional<bool>(true);

api->blockUserFromComment(tenantId, commentId, params, options)
    .then([](std::shared_ptr<BlockSuccess> result){ })
    .then([](pplx::task<void> t){ try { t.get(); } catch (const std::exception&) { } });
[inline-code-end]