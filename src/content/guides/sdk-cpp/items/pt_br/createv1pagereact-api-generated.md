## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| title | string | Não |  |

## Resposta

Retorna: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo createV1PageReact'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto urlId = utility::string_t(U("https://example.com/articles/789"));
boost::optional<utility::string_t> title = utility::string_t(U("Understanding Async C++"));
api->createV1PageReact(tenantId, urlId, title).then([](pplx::task<std::shared_ptr<CreateV1PageReact>> task) {
    try {
        auto response = task.get();
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---