## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| userId | string | 아니요 |  |
| anonUserId | string | 아니요 |  |

## 응답

반환: [`FlagComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagComment_200_response.h)

## 예제

[inline-code-attrs-start title = 'unFlagComment 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654321");
boost::optional<utility::string_t> userId(U("user@example.com"));
boost::optional<utility::string_t> anonUserId;
auto fallback = std::make_shared<FlagComment_200_response>();
api->unFlagComment(tenantId, commentId, userId, anonUserId)
    .then([fallback](pplx::task<std::shared_ptr<FlagComment_200_response>> t) {
        try {
            auto resp = t.get();
            if (!resp) resp = fallback;
        } catch (...) {
        }
    });
[inline-code-end]

---