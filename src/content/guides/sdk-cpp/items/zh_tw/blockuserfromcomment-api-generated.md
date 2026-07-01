## 參數

| 名稱 | 類型 | 必須 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| blockFromCommentParams | BlockFromCommentParams | 是 |  |
| options | const BlockUserFromCommentOptions& | 是 |  |

## 回應

返回：[`BlockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BlockSuccess.h)

## 範例

[inline-code-attrs-start title = 'blockUserFromComment 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("comment-789");
BlockFromCommentParams params;
params.reason = utility::conversions::to_string_t("Inappropriate content");
params.durationDays = boost::optional<int>(30);
BlockUserFromCommentOptions options;
options.notifyUser = boost::optional<bool>(true);

api->blockUserFromComment(tenantId, commentId, params, options)
    .then([](std::shared_ptr<BlockSuccess> result){ })
    .then([](pplx::task<void> t){ try { t.get(); } catch (const std::exception&) { } });
[inline-code-end]