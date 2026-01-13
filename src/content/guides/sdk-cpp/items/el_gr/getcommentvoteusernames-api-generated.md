## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| dir | int32_t | Ναι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetCommentVoteUserNames_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentVoteUserNames_200_response.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentVoteUserNames'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("cmt-456");
int32_t dir = 1;
boost::optional<utility::string_t> sso{ utility::string_t(U("user@example.com")) };
api->getCommentVoteUserNames(tenantId, commentId, dir, sso)
    .then([](pplx::task<std::shared_ptr<GetCommentVoteUserNames_200_response>> t) {
        try {
            auto resp = t.get();
            if (!resp) resp = std::make_shared<GetCommentVoteUserNames_200_response>();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]

---