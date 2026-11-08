## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetCommentBanStatusResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getCommentBanStatus उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U"my-tenant-123");
auto commentId = utility::string_t(U"comment-456");
boost::optional<utility::string_t> sso = utility::string_t(U"user@example.com");

api->getCommentBanStatus(tenantId, commentId, sso).then([](pplx::task<std::shared_ptr<GetCommentBanStatusResponse>> t){
    try{
        auto response = t.get();
    }catch(const std::exception&){ }
});
[inline-code-end]