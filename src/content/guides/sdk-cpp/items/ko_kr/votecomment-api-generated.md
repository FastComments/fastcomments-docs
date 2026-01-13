## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| urlId | string | 예 |  |
| broadcastId | string | 예 |  |
| voteBodyParams | VoteBodyParams | 예 |  |
| sessionId | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`VoteComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/VoteComment_200_response.h)

## 예제

[inline-code-attrs-start title = 'voteComment 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
VoteBodyParams voteBodyParams;
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-7890");
utility::string_t urlId = U("/articles/2025/how-to-cpprest");
utility::string_t broadcastId = U("broadcast-321");
boost::optional<utility::string_t> sessionId = boost::optional<utility::string_t>(U("sess-0a1b2c"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-jwt-xyz"));

api->voteComment(tenantId, commentId, urlId, broadcastId, voteBodyParams, sessionId, sso)
    .then([](pplx::task<std::shared_ptr<VoteComment_200_response>> t) {
        try {
            auto resp = t.get();
            if (!resp) resp = std::make_shared<VoteComment_200_response>();
        } catch (...) {
        }
    });
[inline-code-end]

---