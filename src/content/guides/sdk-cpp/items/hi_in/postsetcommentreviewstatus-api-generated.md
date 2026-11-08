## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| commentId | string | हाँ |  |
| options | const PostSetCommentReviewStatusOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## उदाहरण

[inline-code-attrs-start title = 'postSetCommentReviewStatus उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("cmt-456789");
PostSetCommentReviewStatusOptions opts;
opts.status = utility::conversions::to_string_t("approved");
opts.note = boost::optional<utility::string_t>(utility::conversions::to_string_t("Looks good"));
api->postSetCommentReviewStatus(tenantId, commentId, opts)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t){
        try{
            auto resp = t.get();
        }catch(const std::exception& e){
        }
    });
[inline-code-end]