## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIGetLogsResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getLogs Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t commentId = U("comment-456");
boost::optional<utility::string_t> sso = U("sso-token-abc");

api->getLogs(tenantId, commentId, sso).then([](pplx::task<std::shared_ptr<ModerationAPIGetLogsResponse>> t){
    try{
        auto response = t.get();
    }catch(...){
    }
});
[inline-code-end]