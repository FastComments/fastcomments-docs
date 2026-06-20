---
## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| spam | bool | 아니오 |  |
| permNotSpam | bool | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예제

[inline-code-attrs-start title = 'postSetCommentSpamStatus 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-7890");
boost::optional<bool> spam = true;
boost::optional<bool> permNotSpam = false;
boost::optional<utility::string_t> sso = U("user@example.com");

api->postSetCommentSpamStatus(commentId, spam, permNotSpam, sso)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task) {
    try {
        auto resp = task.get();
        auto ack = std::make_shared<APIEmptyResponse>();
        if (resp) *ack = *resp;
        return ack;
    } catch (...) {
        return std::make_shared<APIEmptyResponse>();
    }
});
[inline-code-end]

---