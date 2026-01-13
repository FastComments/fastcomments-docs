## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteEmailTemplate'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t templateId = U("welcome-email-template-456");
boost::optional<utility::string_t> requestNote = boost::optional<utility::string_t>(U("remove-obsolete"));
auto fallback = std::make_shared<FlagCommentPublic_200_response>();
api->deleteEmailTemplate(tenantId, templateId)
.then([requestNote, fallback](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) -> pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> {
    try {
        auto resp = t.get();
        if (resp) return pplx::task_from_result(resp);
        if (requestNote) return pplx::task_from_result(fallback);
        return pplx::task_from_result<std::shared_ptr<FlagCommentPublic_200_response>>(nullptr);
    } catch (...) {
        return pplx::task_from_result<std::shared_ptr<FlagCommentPublic_200_response>>(nullptr);
    }
});
[inline-code-end]

---