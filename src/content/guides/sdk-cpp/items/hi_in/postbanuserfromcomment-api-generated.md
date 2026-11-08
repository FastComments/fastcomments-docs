## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| options | const PostBanUserFromCommentOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/BanUserFromCommentResult.h)

## उदाहरण

[inline-code-attrs-start title = 'postBanUserFromComment उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456789");
PostBanUserFromCommentOptions options;
options.reason = boost::optional<utility::string_t>(U("spam"));
options.durationDays = boost::optional<int>(30);

api->postBanUserFromComment(tenantId, commentId, options)
    .then([](std::shared_ptr<BanUserFromCommentResult> result) {
        // result handling
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception& e) { /* error handling */ }
    });
[inline-code-end]