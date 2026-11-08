## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| options | const PostRemoveCommentOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PostRemoveCommentApiResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'postRemoveComment उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto commentId = utility::string_t(U("cmt-456789"));
PostRemoveCommentOptions options;
options.permanent = boost::optional<bool>(true);
api->postRemoveComment(tenantId, commentId, options)
    .then([](pplx::task<std::shared_ptr<PostRemoveCommentApiResponse>> task) {
        try {
            auto response = task.get();
            // प्रतिक्रिया प्रोसेस करें
        } catch (const std::exception& ex) {
            // त्रुटि को हैंडल करें
        }
    });
[inline-code-end]