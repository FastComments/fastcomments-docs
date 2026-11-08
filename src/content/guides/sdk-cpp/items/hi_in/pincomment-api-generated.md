## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| broadcastId | string | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`ChangeCommentPinStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/ChangeCommentPinStatusResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'pinComment उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456");
utility::string_t broadcastId = U("broadcast-789");
boost::optional<utility::string_t> sso = U("sso-token-abc");

api->pinComment(tenantId, commentId, broadcastId, sso)
    .then([](pplx::task<std::shared_ptr<ChangeCommentPinStatusResponse>> task) {
        try {
            auto response = task.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]