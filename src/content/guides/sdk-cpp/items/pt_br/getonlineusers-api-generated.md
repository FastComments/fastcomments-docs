Visualizadores atualmente online de uma página: pessoas cuja sessão websocket está assinada à página neste momento.  
Retorna anonCount + totalCount (assinantes de toda a sala, incluindo visualizadores anônimos que não enumeramos).

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| options | const GetOnlineUsersOptions& | Sim |  |

## Resposta

Retorna: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOnlineResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getOnlineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("article-456");
auto options = std::make_shared<GetOnlineUsersOptions>();
options->maxResults = boost::optional<int>(100);
options->includeInactive = boost::optional<bool>(false);
api->getOnlineUsers(tenantId, urlId, *options).then([](pplx::task<std::shared_ptr<PageUsersOnlineResponse>> task){
    try{
        auto response = task.get();
    }catch(const std::exception&){
    }
});
[inline-code-end]