## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const PostSetCommentApprovalStatusOptions& | Yes |  |

## 응답

반환: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SetCommentApprovedResponse.h)

## 예시

[inline-code-attrs-start title = 'postSetCommentApprovalStatus 예시'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("comment-abc123");
auto options = std::make_shared<PostSetCommentApprovalStatusOptions>();
options->approved = boost::optional<bool>(true);
options->reason = boost::optional<utility::string_t>(utility::conversions::to_string_t("Inappropriate content"));
api->postSetCommentApprovalStatus(tenantId, commentId, *options)
    .then([](pplx::task<std::shared_ptr<SetCommentApprovedResponse>> task) {
        try {
            auto response = task.get();
        } catch (const std::exception& ex) {
        }
    });
[inline-code-end]