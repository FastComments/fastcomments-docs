Popis stranica za najmodavca. Koristi FChat desktop klijent za popunjavanje popisa soba.  
Potrebno je da `enableFChat` bude postavljen na true u razriješenoj prilagođenoj konfiguraciji za svaku stranicu.  
Stranice koje zahtijevaju SSO filtriraju se prema pristupu grupi korisnika koji je zatražio.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| options | const GetPagesPublicOptions& | Da |  |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Primjer

[inline-code-attrs-start title = 'Primjer getPagesPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // process response if needed
    }catch(const std::exception&){
        // handle error if needed
    }
});
[inline-code-end]