## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| banEmail | bool | Nein |  |
| banEmailDomain | bool | Nein |  |
| banIP | bool | Nein |  |
| deleteAllUsersComments | bool | Nein |  |
| bannedUntil | string | Nein |  |
| isShadowBan | bool | Nein |  |
| updateId | string | Nein |  |
| banReason | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BanUserFromCommentResult.h)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für postBanUserFromComment'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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