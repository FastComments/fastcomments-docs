## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| dir | int32_t | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetCommentVoteUserNamesSuccessResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'getCommentVoteUserNames उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto task = api->getCommentVoteUserNames(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("comment-456"),
    static_cast<int32_t>(1),
    boost::optional<utility::string_t>(utility::conversions::to_string_t("sso-token"))
).then([](pplx::task<std::shared_ptr<GetCommentVoteUserNamesSuccessResponse>> t){
    try{
        auto response = t.get();
    }catch(const std::exception&){ }
});
[inline-code-end]