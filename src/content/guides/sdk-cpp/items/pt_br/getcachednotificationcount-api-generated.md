## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |

## Resposta

Retorna: [`GetCachedNotificationCount_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCachedNotificationCount_200_response.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getCachedNotificationCount'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> opt_status = boost::optional<utility::string_t>(U("unread"));
api->getCachedNotificationCount(tenantId, userId)
    .then([=](pplx::task<std::shared_ptr<GetCachedNotificationCount_200_response>> t){
        try {
            std::shared_ptr<GetCachedNotificationCount_200_response> resp = t.get();
            if(!resp) return;
            auto copy = std::make_shared<GetCachedNotificationCount_200_response>(*resp);
        } catch(const std::exception &){
        }
    });
[inline-code-end]

---