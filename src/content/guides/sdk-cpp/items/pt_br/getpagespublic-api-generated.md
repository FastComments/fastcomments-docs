Lista páginas para um tenant. Usada pelo cliente desktop FChat para popular sua lista de salas.
Requer que `enableFChat` seja true na configuração customizada resolvida de cada página.
Páginas que exigem SSO são filtradas de acordo com o acesso de grupo do usuário solicitante.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| options | const GetPagesPublicOptions& | Sim |  |

## Resposta

Retorna: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getPagesPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // process response if needed
    }catch(const std::exception&){
        // handle error if needed
    }
});
[inline-code-end]