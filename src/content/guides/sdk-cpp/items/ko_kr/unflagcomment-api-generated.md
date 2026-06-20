## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| userId | string | 아니오 |  |
| anonUserId | string | 아니오 |  |

## 응답

반환: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentResponse.h)

## 예제

[inline-code-attrs-start title = 'unFlagComment 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-7890");
boost::optional<utility::string_t> userId = boost::optional<utility::string_t>(U("user@example.com"));
boost::optional<utility::string_t> anonUserId;
api->unFlagComment(tenantId, commentId, userId, anonUserId)
.then([](pplx::task<std::shared_ptr<FlagCommentResponse>> t) {
    try {
        auto resp = t.get();
        auto fallback = std::make_shared<FlagCommentResponse>();
        if (!resp) resp = fallback;
        (void)resp;
    } catch (const std::exception &e) {
        (void)e;
    }
});
[inline-code-end]

---