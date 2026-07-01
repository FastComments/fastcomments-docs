## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updatableCommentParams | UpdatableCommentParams | Yes |  |
| options | const UpdateCommentOptions& | Yes |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## דוגמה

[inline-code-attrs-start title = 'updateComment דוגמה'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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