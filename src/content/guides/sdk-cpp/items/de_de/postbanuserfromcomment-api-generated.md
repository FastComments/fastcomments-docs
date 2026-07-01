## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | const PostBanUserFromCommentOptions& | Yes |  |

## Antwort

Rückgabe: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BanUserFromCommentResult.h)

## Beispiel

[inline-code-attrs-start title = 'postBanUserFromComment Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456789");
PostBanUserFromCommentOptions options;
options.reason = boost::optional<utility::string_t>(U("spam"));
options.durationDays = boost::optional<int>(30);

api->postBanUserFromComment(tenantId, commentId, options)
    .then([](std::shared_ptr<BanUserFromCommentResult> result) {
        // Ergebnisverarbeitung
    })
    .then([](pplx::task<void> t) {
        try { t.get(); } catch (const std::exception& e) { /* Fehlerbehandlung */ }
    });
[inline-code-end]