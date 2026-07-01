## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |

## Respuesta

Devuelve: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetVotesResponse.h)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getVotes'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto urlId = utility::string_t(U("article-456"));
boost::optional<utility::string_t> extraHeader = boost::none;

api->getVotes(tenantId, urlId).then([=](pplx::task<std::shared_ptr<GetVotesResponse>> task) {
    try {
        auto original = task.get();
        auto response = std::make_shared<GetVotesResponse>(*original);
    } catch (...) {
        // manejo de errores
    }
});
[inline-code-end]

---