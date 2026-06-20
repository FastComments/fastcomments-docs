## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| urlId | string | כן |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-putCloseThread'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t urlId = utility::conversions::to_string_t("my-tenant-123/thread-98765");
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("user@example.com");
api->putCloseThread(urlId, sso)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task) {
    try {
        auto resp = task.get();
        if (!resp) resp = std::make_shared<APIEmptyResponse>();
        return resp;
    } catch (const std::exception&) {
        return std::make_shared<APIEmptyResponse>();
    }
});
[inline-code-end]

---