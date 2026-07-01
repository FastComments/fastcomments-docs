## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIChildCommentsResponse.h)

## דוגמה

[inline-code-attrs-start title = 'getCommentChildren דוגמה'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456");
boost::optional<utility::string_t> sso = U("sso-token-abc");

api->getCommentChildren(tenantId, commentId, sso)
    .then([](pplx::task<std::shared_ptr<ModerationAPIChildCommentsResponse>> task) {
        try {
            auto response = task.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]