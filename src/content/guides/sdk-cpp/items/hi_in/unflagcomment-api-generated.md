## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| options | const UnFlagCommentOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/FlagCommentResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'unFlagComment उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = UnFlagCommentOptions{};
options.reason = boost::optional<utility::string_t>(U("Resolved by moderator"));
api->unFlagComment(U("my-tenant-123"), U("comment-456"), options)
    .then([](std::shared_ptr<FlagCommentResponse> response) {
        if (response) {
            auto status = response->status;
            // आवश्यक हो तो status प्रोसेस करें
        }
    })
    .then([](pplx::task<void> previous) {
        try {
            previous.get();
        } catch (const std::exception& e) {
            // त्रुटि को संभालें
        }
    });
[inline-code-end]