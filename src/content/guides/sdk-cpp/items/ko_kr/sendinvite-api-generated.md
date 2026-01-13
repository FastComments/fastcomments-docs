---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| fromName | string | 예 |  |

## 응답

반환: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 예제

[inline-code-attrs-start title = 'sendInvite 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("comment-456");
utility::string_t fromName = U("moderator@example.com");
boost::optional<utility::string_t> inviteNote = boost::optional<utility::string_t>(U("Please review this flagged comment"));
auto task = api->sendInvite(tenantId, id, fromName)
    .then([inviteNote](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) -> std::shared_ptr<FlagCommentPublic_200_response> {
        try {
            auto resp = t.get();
            auto wrapped = std::make_shared<FlagCommentPublic_200_response>(*resp);
            (void)inviteNote;
            return wrapped;
        } catch (...) {
            throw;
        }
    });
[inline-code-end]

---