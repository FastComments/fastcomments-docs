## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| page | int32_t | Não |  |
| limit | int32_t | Não |  |
| skip | int32_t | Não |  |
| asTree | bool | Não |  |
| skipChildren | int32_t | Não |  |
| limitChildren | int32_t | Não |  |
| maxTreeDepth | int32_t | Não |  |
| urlId | string | Não |  |
| userId | string | Não |  |
| anonUserId | string | Não |  |
| contextUserId | string | Não |  |
| hashTag | string | Não |  |
| parentId | string | Não |  |
| direction | SortDirections | Não |  |
| fromDate | int64_t | Não |  |
| toDate | int64_t | Não |  |

## Resposta

Retorna: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetCommentsResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getComments'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId(U("my-tenant-123"));
boost::optional<int32_t> page = 1;
boost::optional<int32_t> limit = 50;
boost::optional<bool> asTree = true;
boost::optional<utility::string_t> userId = utility::string_t(U("user@example.com"));
boost::optional<int64_t> fromDate = 1622505600LL;
boost::optional<int64_t> toDate = 1625097600LL;

api->getComments(tenantId,
                 page,
                 limit,
                 boost::optional<int32_t>(),
                 asTree,
                 boost::optional<int32_t>(),
                 boost::optional<int32_t>(),
                 boost::optional<int32_t>(),
                 boost::optional<utility::string_t>(),
                 userId,
                 boost::optional<utility::string_t>(),
                 boost::optional<utility::string_t>(),
                 boost::optional<utility::string_t>(),
                 boost::optional<utility::string_t>(),
                 boost::optional<SortDirections>(),
                 fromDate,
                 toDate)
.then([](pplx::task<std::shared_ptr<APIGetCommentsResponse>> t){
    try {
        auto resp = t.get();
        auto holder = std::make_shared<std::shared_ptr<APIGetCommentsResponse>>(resp);
        (void)holder;
    } catch (...) {}
});
[inline-code-end]

---