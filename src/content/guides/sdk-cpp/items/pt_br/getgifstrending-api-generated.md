## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| locale | string | Não |  |
| rating | string | Não |  |
| page | double | Não |  |

## Resposta

Retorna: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetGifsTrendingResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getGifsTrending'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> locale = boost::optional<utility::string_t>(U("en-US"));
boost::optional<utility::string_t> rating = boost::optional<utility::string_t>(U("pg"));
boost::optional<double> page = boost::optional<double>(2.0);
auto defaultResp = std::make_shared<GetGifsTrendingResponse>();
api->getGifsTrending(tenantId, locale, rating, page)
    .then([defaultResp](std::shared_ptr<GetGifsTrendingResponse> resp) {
        auto r = resp ? resp : defaultResp;
        std::cout << "Received trending GIFs response pointer: " << static_cast<const void*>(r.get()) << std::endl;
    });
[inline-code-end]

---