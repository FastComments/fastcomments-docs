## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updatableCommentParams | UpdatableCommentParams | はい |  |
| options | const UpdateCommentOptions& | はい |  |

## レスポンス

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'updateComment の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t("my-tenant-123");
auto commentId = utility::string_t("cmt-789");
UpdatableCommentParams updatable;
updatable.body = utility::string_t("Edited comment content");
updatable.isSpam = boost::optional<bool>(false);
UpdateCommentOptions options;
options.notify = boost::optional<bool>(true);
api->updateComment(tenantId, commentId, updatable, options)
    .then([](std::shared_ptr<APIEmptyResponse>) {})
    .then([](pplx::task<void> t) { try { t.get(); } catch(const std::exception&) {} });
[inline-code-end]