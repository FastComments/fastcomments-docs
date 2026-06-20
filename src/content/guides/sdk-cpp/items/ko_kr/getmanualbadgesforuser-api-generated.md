## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| badgesUserId | string | 아니오 |  |
| commentId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserManualBadgesResponse.h)

## 예제

[inline-code-attrs-start title = 'getManualBadgesForUser 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> badgesUserId(boost::optional<utility::string_t>(U("user@example.com")));
boost::optional<utility::string_t> commentId(boost::optional<utility::string_t>(U("cmt-789")));
boost::optional<utility::string_t> sso(boost::optional<utility::string_t>(U("my-tenant-123|sso-token-abc")));
auto task = api->getManualBadgesForUser(badgesUserId, commentId, sso)
.then([](pplx::task<std::shared_ptr<GetUserManualBadgesResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<GetUserManualBadgesResponse>();
        return resp;
    } catch (...) {
        return std::make_shared<GetUserManualBadgesResponse>();
    }
});
[inline-code-end]

---