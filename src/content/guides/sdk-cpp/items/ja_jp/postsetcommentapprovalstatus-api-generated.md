## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const PostSetCommentApprovalStatusOptions& | Yes |  |

## レスポンス

戻り値: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SetCommentApprovedResponse.h)

## 例

[inline-code-attrs-start title = 'postSetCommentApprovalStatus の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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