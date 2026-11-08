## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| adjustCommentVotesParams | AdjustCommentVotesParams | हाँ |  |
| options | const PostAdjustCommentVotesOptions& | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/AdjustVotesResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'postAdjustCommentVotes उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("cmt-456789");
AdjustCommentVotesParams params;
params.voteDelta = 1;
params.userIdentifier = utility::conversions::to_string_t("user@example.com");
params.reason = boost::optional<utility::string_t>(utility::conversions::to_string_t("Helpful"));
PostAdjustCommentVotesOptions opts;
opts.timeout = boost::optional<int>(30);
api->postAdjustCommentVotes(tenantId, commentId, params, opts).then([](pplx::task<std::shared_ptr<AdjustVotesResponse>> t){
    try{
        auto resp = t.get();
    }catch(const std::exception& e){
    }
});
[inline-code-end]