## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Da |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Da |  |
| options | const UnBlockUserFromCommentOptions& | Da |  |

## Odgovor

Vraća: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UnblockSuccess.h)

## Primjer

[inline-code-attrs-start title = 'unBlockUserFromComment Primjer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto params = std::make_shared<UnBlockFromCommentParams>();
params->commentId = U("cmt-12345");
params->reason = U("resolved");
UnBlockUserFromCommentOptions opts;
opts.notifyUser = boost::optional<bool>(true);
api->unBlockUserFromComment(U("my-tenant-123"), U("user-456"), *params, opts)
    .then([](pplx::task<std::shared_ptr<UnblockSuccess>> t){
        try{
            auto result = t.get();
        }catch(...){}
    });
[inline-code-end]