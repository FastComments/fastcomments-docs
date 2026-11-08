## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | हाँ |  |
| options | const UnBlockUserFromCommentOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/UnblockSuccess.h)

## उदाहरण

[inline-code-attrs-start title = 'unBlockUserFromComment उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto params = std::make_shared<UnBlockFromCommentParams>();
params->commentId = U("cmt-12345");
params->reason = U("resolved");
UnBlockUserFromCommentOptions opts;
opts.notifyUser = boost::optional<bool>(true);
api->unBlockUserFromComment(U("my-tenant-123"), U("user-456"), *params, opts)
    .then([](pplx::task<std::shared_ptr<UnblockSuccess>> t){
        try{
            auto result = t.get();
        }catch(...){}
    });
[inline-code-end]