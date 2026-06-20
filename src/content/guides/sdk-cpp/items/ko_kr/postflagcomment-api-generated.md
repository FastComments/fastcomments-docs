## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예제

[inline-code-attrs-start title = 'postFlagComment 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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