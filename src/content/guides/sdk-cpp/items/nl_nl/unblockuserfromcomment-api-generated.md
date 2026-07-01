## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Ja |  |
| options | const UnBlockUserFromCommentOptions& | Ja |  |

## Response

Retourneert: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UnblockSuccess.h)

## Voorbeeld

[inline-code-attrs-start title = 'unBlockUserFromComment Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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