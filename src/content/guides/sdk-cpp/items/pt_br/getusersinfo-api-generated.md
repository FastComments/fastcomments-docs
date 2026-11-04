Informações em lote de usuários para um tenant. Dados os userIds, retorna as informações de exibição de User / SSOUser.
Usado pelo widget de comentários para enriquecer usuários que acabaram de aparecer via um evento de presença.
Sem contexto de página: a privacidade é aplicada de forma uniforme (perfis privados são mascarados).

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| ids | string | Sim |  |

## Resposta

Retorna: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/PageUsersInfoResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getUsersInfo'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ids = U("alice@example.com,bob@example.com");
boost::optional<utility::string_t> locale = boost::make_optional(U("en-US"));

api->getUsersInfo(tenantId, ids).then([](pplx::task<std::shared_ptr<PageUsersInfoResponse>> t){
    try{
        auto response = t.get();
        // processar a resposta
    }catch(const std::exception&){
        // tratar o erro
    }
});
[inline-code-end]