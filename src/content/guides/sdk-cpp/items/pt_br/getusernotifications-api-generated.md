## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Não |  |
| pageSize | int32_t | Não |  |
| afterId | string | Não |  |
| includeContext | bool | Não |  |
| afterCreatedAt | int64_t | Não |  |
| unreadOnly | bool | Não |  |
| dmOnly | bool | Não |  |
| noDm | bool | Não |  |
| includeTranslations | bool | Não |  |
| includeTenantNotifications | bool | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetMyNotificationsResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getUserNotifications'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
api->getUserNotifications(
    tenantId,
    boost::optional<utility::string_t>(U("post-456")),
    boost::optional<int32_t>(50),
    boost::optional<utility::string_t>(U("notif-789")),
    boost::optional<bool>(true),
    boost::optional<int64_t>(1625097600000LL),
    boost::optional<bool>(true),
    boost::optional<bool>(false),
    boost::optional<bool>(false),
    boost::optional<bool>(true),
    boost::optional<bool>(false),
    boost::optional<utility::string_t>(U("user@example.com"))
).then([](pplx::task<std::shared_ptr<GetMyNotificationsResponse>> t){
    try {
        auto resp = t.get();
        if(!resp) resp = std::make_shared<GetMyNotificationsResponse>();
        // use resp, por exemplo, inspecione os campos
    } catch(const std::exception &e) {
    }
});
[inline-code-end]