---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| commentId | string | Evet |  |
| banEmail | bool | Hayır |  |
| banEmailDomain | bool | Hayır |  |
| banIP | bool | Hayır |  |
| deleteAllUsersComments | bool | Hayır |  |
| bannedUntil | string | Hayır |  |
| isShadowBan | bool | Hayır |  |
| updateId | string | Hayır |  |
| banReason | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BanUserFromCommentResult.h)

## Örnek

[inline-code-attrs-start title = 'postBanUserFromComment Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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