## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| options | const DeleteCommentOptions& | 예 |  |

## 응답

반환: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteCommentResult.h)

## 예시

[inline-code-attrs-start title = 'deleteComment 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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