## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIGetLogsResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getLogs'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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