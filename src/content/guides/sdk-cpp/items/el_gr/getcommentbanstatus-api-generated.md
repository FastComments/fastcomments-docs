## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentBanStatusResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentBanStatus'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U"my-tenant-123");
auto commentId = utility::string_t(U"comment-456");
boost::optional<utility::string_t> sso = utility::string_t(U"user@example.com");

api->getCommentBanStatus(tenantId, commentId, sso).then([](pplx::task<std::shared_ptr<GetCommentBanStatusResponse>> t){
    try{
        auto response = t.get();
    }catch(const std::exception&){ }
});
[inline-code-end]