## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| r_namespace | string | Sim |  |
| component | string | Sim |  |
| locale | string | Não |  |
| useFullTranslationIds | bool | Não |  |

## Resposta

Retorna: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTranslationsResponse.h)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getTranslations'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> locale(U("en-US"));
boost::optional<bool> useFullTranslationIds(true);
api->getTranslations(U("my-tenant-123"), U("comment-widget"), locale, useFullTranslationIds)
.then([](pplx::task<std::shared_ptr<GetTranslationsResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto copied = std::make_shared<GetTranslationsResponse>(*resp);
        }
    } catch (const std::exception&) {
        throw;
    }
});
[inline-code-end]

---