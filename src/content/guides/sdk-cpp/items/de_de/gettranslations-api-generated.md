---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| r_namespace | string | Ja |  |
| component | string | Ja |  |
| locale | string | Nein |  |
| useFullTranslationIds | bool | Nein |  |

## Antwort

Gibt zurück: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTranslationsResponse.h)

## Beispiel

[inline-code-attrs-start title = 'getTranslations Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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