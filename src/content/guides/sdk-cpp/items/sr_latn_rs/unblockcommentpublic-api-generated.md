## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| commentId | string | Da |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Da |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UnblockSuccess.h)

## Primer

[inline-code-attrs-start title = 'unBlockCommentPublic Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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