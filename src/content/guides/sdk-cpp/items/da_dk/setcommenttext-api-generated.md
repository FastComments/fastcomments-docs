## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Ja |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Ja |  |
| editKey | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicAPISetCommentTextResponse.h)

## Eksempel

[inline-code-attrs-start title = 'setCommentText Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456");
utility::string_t broadcastId = U("broadcast-789");
CommentTextUpdateRequest commentTextUpdateRequest;
commentTextUpdateRequest.text = U("Updated comment: clarified wording and fixed a typo.");
boost::optional<utility::string_t> editKey = boost::optional<utility::string_t>(U("editKey-abc123"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->setCommentText(tenantId, commentId, broadcastId, commentTextUpdateRequest, editKey, sso)
    .then([](pplx::task<std::shared_ptr<PublicAPISetCommentTextResponse>> task) {
        try {
            auto resp = task.get();
            if (!resp) resp = std::make_shared<PublicAPISetCommentTextResponse>();
        } catch (const std::exception&) {}
    });
[inline-code-end]

---