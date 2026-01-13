## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| redirectURL | string | 아니요 |  |

## 응답

반환: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## 예제

[inline-code-attrs-start title = 'sendLoginLink 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> redirectURL = utility::string_t(U("https://app.example.com/welcome"));
api->sendLoginLink(tenantId, userId, redirectURL)
    .then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t)
    {
        try
        {
            auto resp = t.get();
            if (resp) return resp;
            return std::make_shared<FlagCommentPublic_200_response>();
        }
        catch (...)
        {
            return std::make_shared<FlagCommentPublic_200_response>();
        }
    });
[inline-code-end]

---