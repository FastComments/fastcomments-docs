## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| banEmail | bool | 아니오 |  |
| banEmailDomain | bool | 아니오 |  |
| banIP | bool | 아니오 |  |
| deleteAllUsersComments | bool | 아니오 |  |
| bannedUntil | string | 아니오 |  |
| isShadowBan | bool | 아니오 |  |
| updateId | string | 아니오 |  |
| banReason | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BanUserFromCommentResult.h)

## 예제

[inline-code-attrs-start title = 'postBanUserFromComment 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("comment-98765");
auto continuation = api->postBanUserFromComment(
    commentId,
    boost::optional<bool>(true),
    boost::optional<bool>(true),
    boost::optional<bool>(false),
    boost::optional<bool>(true),
    boost::optional<utility::string_t>(U("2026-12-31T23:59:59Z")),
    boost::optional<bool>(false),
    boost::optional<utility::string_t>(U("update-abc123")),
    boost::optional<utility::string_t>(U("User posted spam links")),
    boost::optional<utility::string_t>(U("sso-token-7f3"))
).then([](pplx::task<std::shared_ptr<BanUserFromCommentResult>> t){
    try {
        auto result = t.get();
        if (!result) result = std::make_shared<BanUserFromCommentResult>();
    } catch (...) {}
});
continuation.wait();
[inline-code-end]

---