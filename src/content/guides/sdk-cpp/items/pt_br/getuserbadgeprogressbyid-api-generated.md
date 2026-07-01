## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |

## Resposta

Retorna: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetUserBadgeProgressResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getUserBadgeProgressById'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto userId = U("user-456");
api->getUserBadgeProgressById(tenantId, userId)
    .then([=](pplx::task<std::shared_ptr<APIGetUserBadgeProgressResponse>> t){
        try{
            auto resp = t.get();
            boost::optional<std::shared_ptr<APIGetUserBadgeProgressResponse>> optResp = resp;
            if(optResp){}
        }catch(const std::exception&){}
    });
[inline-code-end]