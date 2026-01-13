## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| userId | string | Sim |  |

## Resposta

Retorna: [`GetUserBadgeProgressById_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserBadgeProgressById_200_response.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getUserBadgeProgressByUserId'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> optUserId = utility::string_t(U("user@example.com"));
api->getUserBadgeProgressByUserId(tenantId, optUserId.value()).then(
    [](pplx::task<std::shared_ptr<GetUserBadgeProgressById_200_response>> t){
        try {
            auto resp = t.get();
            auto copy = std::make_shared<GetUserBadgeProgressById_200_response>(*resp);
        } catch (const std::exception&) {
        }
    }
);
[inline-code-end]

---