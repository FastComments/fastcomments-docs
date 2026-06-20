## Parametri

| Name | Type | Obbligatorio | Description |
|------|------|--------------|-------------|
| commentId | string | Sì |  |
| banEmail | bool | No |  |
| banEmailDomain | bool | No |  |
| banIP | bool | No |  |
| deleteAllUsersComments | bool | No |  |
| bannedUntil | string | No |  |
| isShadowBan | bool | No |  |
| updateId | string | No |  |
| banReason | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BanUserFromCommentResult.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di postBanUserFromComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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