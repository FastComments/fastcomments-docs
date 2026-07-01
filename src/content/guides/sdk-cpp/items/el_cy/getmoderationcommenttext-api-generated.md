## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| sso | string | No |  |

## Απόκριση

Επιστρέφει: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentTextResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getModerationCommentText'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-987654");
boost::optional<utility::string_t> sso = U("sso-token-abc");

api->getModerationCommentText(tenantId, commentId, sso)
    .then([](pplx::task<std::shared_ptr<GetCommentTextResponse>> t) {
        try {
            auto resp = t.get();
            auto text = std::make_shared<std::string>(resp->commentText);
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---