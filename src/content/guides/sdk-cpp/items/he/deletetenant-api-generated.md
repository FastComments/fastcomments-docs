## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| sure | string | לא |  |

## תגובה

מחזיר: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteTenant'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("flag-456");
boost::optional<utility::string_t> sure = utility::string_t(U("true"));
api->deleteTenant(tenantId, id, sure)
    .then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) {
        try {
            auto resp = t.get();
            auto safeResp = resp ? resp : std::make_shared<FlagCommentPublic_200_response>();
            (void)safeResp;
        } catch (...) {}
    });
[inline-code-end]

---