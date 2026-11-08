## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| options | const DeleteCommentOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/DeleteCommentResult.h)

## उदाहरण

[inline-code-attrs-start title = 'deleteComment उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("comment-789");
DeleteCommentOptions options;
options.reason = boost::optional<utility::string_t>(utility::conversions::to_string_t("Inappropriate content"));
options.force = boost::optional<bool>(true);
api->deleteComment(tenantId, commentId, options).then([](pplx::task<std::shared_ptr<DeleteCommentResult>> task){
    try{
        auto result = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]