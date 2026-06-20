## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|-------------|-------------|
| commentId | string | Ja |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Beispiel

[inline-code-attrs-start title = 'postUnFlagComment Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-8f3a21b7");
boost::optional<utility::string_t> sso = utility::string_t(U("user@example.com"));
api->postUnFlagComment(commentId, sso)
.then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<APIEmptyResponse>();
    } catch (const std::exception &e) {
    }
}).wait();
[inline-code-end]

---