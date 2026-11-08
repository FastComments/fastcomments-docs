## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createCommentParams | CreateCommentParams | हाँ |  |
| options | const SaveCommentOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/APISaveCommentResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'saveComment उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
CreateCommentParams commentParams;
commentParams.body = utility::string_t(U("Great article!"));
commentParams.author = utility::string_t(U("jane.doe@example.com"));
commentParams.parentId = boost::optional<utility::string_t>(utility::string_t(U("parent-789")));

SaveCommentOptions options;
options.preview = boost::optional<bool>(false);

api->saveComment(utility::string_t(U("my-tenant-123")), commentParams, options)
    .then([](std::shared_ptr<APISaveCommentResponse> response) {
        auto commentId = response->commentId;
    });
[inline-code-end]