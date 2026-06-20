## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| broadcastId | string | Ναι |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Ναι |  |
| editKey | string | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`PublicAPISetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicAPISetCommentTextResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα setCommentText'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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