## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| setCommentTextParams | SetCommentTextParams | Ja |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SetCommentTextResponse.h)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für postSetCommentText'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t commentId = U("cmt-9a8b7c6d");
auto paramsPtr = std::make_shared<SetCommentTextParams>();
paramsPtr->text = U("Updated comment: I've rephrased for clarity.");
paramsPtr->editedBy = U("moderator@example.com");
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("sso-token-abc123"));

api->postSetCommentText(commentId, *paramsPtr, sso)
    .then([](pplx::task<std::shared_ptr<SetCommentTextResponse>> task) {
        try {
            auto resp = task.get();
            if (resp) (void)resp;
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---