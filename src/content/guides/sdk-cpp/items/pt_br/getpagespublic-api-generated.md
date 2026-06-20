Lista as páginas de um tenant. Usado pelo cliente desktop FChat para preencher sua lista de salas.
Requer que `enableFChat` seja true na configuração personalizada resolvida para cada página.
Páginas que requerem SSO são filtradas com base no acesso de grupo do usuário solicitante.

## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| cursor | string | Não |  |
| limit | int32_t | Não |  |
| q | string | Não |  |
| sortBy | PagesSortBy | Não |  |
| hasComments | bool | Não |  |

## Resposta

Retorna: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getPagesPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> cursor = utility::string_t(U("cursor_abc"));
boost::optional<int32_t> limit = 50;
boost::optional<utility::string_t> q = utility::string_t(U("status:published"));
boost::optional<PagesSortBy> sortBy = PagesSortBy::NEWEST;
boost::optional<bool> hasComments = true;
api->getPagesPublic(tenantId, cursor, limit, q, sortBy, hasComments)
.then([](std::shared_ptr<GetPublicPagesResponse> resp){
    if (!resp) resp = std::make_shared<GetPublicPagesResponse>();
})
.wait();
[inline-code-end]

---