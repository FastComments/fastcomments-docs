## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| commentId | string | כן |  |
| sso | string | לא |  |

## תשובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## דוגמה

[inline-code-attrs-start title = 'postFlagComment דוגמה'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = utility::conversions::to_string_t("cmt-8f3a2b9e");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(utility::conversions::to_string_t("user@example.com"));
api->postFlagComment(commentId, sso).then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task){
    try {
        std::shared_ptr<APIEmptyResponse> resp = task.get();
        if (resp) {
            auto copy = std::make_shared<APIEmptyResponse>(*resp);
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---