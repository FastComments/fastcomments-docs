## 매개변수

| Name | Type | 필수 | 설명 |
|------|------|------|-------------|
| commentId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentBanStatusResponse.h)

## 예제

[inline-code-attrs-start title = 'getCommentBanStatus 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> sso = utility::string_t(U("user@example.com"));
utility::string_t commentId = utility::string_t(U("comment-98765"));

api->getCommentBanStatus(commentId, sso)
    .then([](pplx::task<std::shared_ptr<GetCommentBanStatusResponse>> task){
        try {
            auto resp = task.get();
            auto copy = std::make_shared<GetCommentBanStatusResponse>(*resp);
            (void)copy;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---