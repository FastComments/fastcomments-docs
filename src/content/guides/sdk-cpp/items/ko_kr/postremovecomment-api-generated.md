## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const PostRemoveCommentOptions& | Yes |  |

## 응답

반환: [`PostRemoveCommentApiResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PostRemoveCommentApiResponse.h)

## 예시

[inline-code-attrs-start title = 'postRemoveComment 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto commentId = utility::string_t(U("cmt-456789"));
PostRemoveCommentOptions options;
options.permanent = boost::optional<bool>(true);
api->postRemoveComment(tenantId, commentId, options)
    .then([](pplx::task<std::shared_ptr<PostRemoveCommentApiResponse>> task) {
        try {
            auto response = task.get();
            // 응답 처리
        } catch (const std::exception& ex) {
            // 오류 처리
        }
    });
[inline-code-end]