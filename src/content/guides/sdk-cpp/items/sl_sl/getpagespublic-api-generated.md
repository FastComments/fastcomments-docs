---
Naštej strani za najemnika. Uporablja ga namizni odjemalec FChat za napolnitev seznama sob.  
Zahteva, da je `enableFChat` nastavljen na true v razrešenih prilagojenih nastavitvah za vsako stran.  
Strani, ki zahtevajo SSO, so filtrirane glede na skupinski dostop zahtevanega uporabnika.

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetPagesPublicOptions& | Yes |  |

## Odgovor

Vrne: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Primer

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // obdelaj odgovor po potrebi
    }catch(const std::exception&){
        // obravnavaj napako po potrebi
    }
});
[inline-code-end]

---