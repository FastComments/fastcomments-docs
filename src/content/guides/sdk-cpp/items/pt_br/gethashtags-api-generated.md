## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| page | double | Não |  |

## Resposta

Retorna: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetHashTagsResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getHashTags'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<double> page = 2.0;

api->getHashTags(tenantId, page).then([](pplx::task<std::shared_ptr<GetHashTagsResponse>> task) {
    try {
        auto resultPtr = task.get();
        auto response = std::make_shared<GetHashTagsResponse>(*resultPtr);
        // usar a resposta
    } catch (const std::exception&) {
        // tratar erro
    }
});
[inline-code-end]