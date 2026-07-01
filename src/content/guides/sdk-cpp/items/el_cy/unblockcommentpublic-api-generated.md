## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Yes |  |
| sso | string | No |  |

## Απόκριση

Επιστρέφει: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UnblockSuccess.h)

## Παράδειγμα

[inline-code-attrs-start title = 'unBlockCommentPublic Παράδειγμα'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto blockParams = PublicBlockFromCommentParams();  
blockParams.reason = U("spam");  
api->unBlockCommentPublic(  
    U("my-tenant-123"),  
    U("comment-789"),  
    blockParams,  
    boost::optional<utility::string_t>(U("sso-token-abc"))  
).then([](std::shared_ptr<UnblockSuccess> result){ });
[inline-code-end]