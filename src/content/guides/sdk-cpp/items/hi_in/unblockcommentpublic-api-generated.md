## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/UnblockSuccess.h)

## उदाहरण

[inline-code-attrs-start title = 'unBlockCommentPublic उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto blockParams = PublicBlockFromCommentParams();  
blockParams.reason = U("spam");  
api->unBlockCommentPublic(  
    U("my-tenant-123"),  
    U("comment-789"),  
    blockParams,  
    boost::optional<utility::string_t>(U("sso-token-abc"))  
).then([](std::shared_ptr<UnblockSuccess> result){ });
[inline-code-end]